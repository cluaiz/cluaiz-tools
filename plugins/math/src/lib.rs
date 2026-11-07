use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct MathRequest {
    expression: String,
    #[serde(default)]
    precision: Option<usize>,
}

#[derive(Serialize)]
struct MathResponse {
    status: String,
    expression: String,
    result: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// Above this, a 64-bit float cannot hold every whole number
const LARGEST_EXACT: f64 = 9007199254740992.0;

#[no_mangle]
pub extern "C" fn allocate(len: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(ptr: *mut u8, len: u32) {
    if !ptr.is_null() && len > 0 {
        let _ = Vec::from_raw_parts(ptr, len as usize, len as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn execute_cel(ptr: *const u8, len: u32) -> u64 {
    let input_bytes = if len > 0 && !ptr.is_null() {
        core::slice::from_raw_parts(ptr, len as usize)
    } else {
        &[]
    };

    let response = match serde_json::from_slice::<MathRequest>(input_bytes) {
        Ok(req) => match evaluate_expression(&req.expression) {
            Ok(val) => {
                let formatted = if let Some(p) = req.precision {
                    Some(format!("{:.prec$}", val, prec = p))
                } else {
                    Some(format!("{}", val))
                };

                MathResponse {
                    status: "success".into(),
                    expression: req.expression,
                    result: Some(val),
                    formatted,
                    note: (val.abs() > LARGEST_EXACT)
                        .then(|| "This result is too large for a 64-bit float to hold exactly".to_string()),
                    error: None,
                }
            }
            Err(e) => MathResponse {
                status: "error".into(),
                expression: req.expression,
                result: None,
                formatted: None,
                note: None,
                error: Some(e),
            },
        },
        Err(_) => MathResponse {
            status: "error".into(),
            expression: String::new(),
            result: None,
            formatted: None,
            note: None,
            error: Some("Invalid input JSON payload. Expected {\"expression\": \"...\"}".into()),
        },
    };

    let output = serde_json::to_vec(&response)
        .unwrap_or_else(|_| Vec::from(b"{\"status\":\"error\",\"error\":\"Serialization failure\"}".as_slice()));

    let out_len = output.len() as u64;
    let out_ptr = output.as_ptr() as u64;
    core::mem::forget(output);

    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}

// ── Recursive Descent Math Evaluator ──

struct Parser<'a> {
    chars: core::iter::Peekable<core::str::Chars<'a>>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn parse_expression(&mut self) -> Result<f64, String> {
        let mut left = self.parse_term()?;
        self.skip_whitespace();
        while let Some(&op) = self.chars.peek() {
            if op == '+' || op == '-' {
                self.chars.next();
                let right = self.parse_term()?;
                if op == '+' {
                    left += right;
                } else {
                    left -= right;
                }
                self.skip_whitespace();
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<f64, String> {
        let mut left = self.parse_power()?;
        self.skip_whitespace();
        while let Some(&op) = self.chars.peek() {
            if op == '*' || op == '/' || op == '%' {
                self.chars.next();
                let right = self.parse_power()?;
                match op {
                    '*' => left *= right,
                    '/' => {
                        if right == 0.0 {
                            return Err("Division by zero".into());
                        }
                        left /= right;
                    }
                    '%' => {
                        if right == 0.0 {
                            return Err("Modulo by zero".into());
                        }
                        left %= right;
                    }
                    _ => unreachable!(),
                }
                self.skip_whitespace();
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<f64, String> {
        self.skip_whitespace();
        // A power binds tighter than a sign: -2^2 is -4
        match self.chars.peek() {
            Some(&'+') => {
                self.chars.next();
                return self.parse_power();
            }
            Some(&'-') => {
                self.chars.next();
                return Ok(-self.parse_power()?);
            }
            _ => {}
        }
        let left = self.parse_factor()?;
        self.skip_whitespace();
        if let Some(&'^') = self.chars.peek() {
            self.chars.next();
            let right = self.parse_power()?; // Right-associative exponentiation
            let value = left.powf(right);
            if value.is_nan() {
                return Err(format!("{left} ^ {right} is not a real number"));
            }
            return Ok(value);
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<f64, String> {
        self.skip_whitespace();
        match self.chars.peek() {
            Some(&'(') => {
                self.chars.next();
                let val = self.parse_expression()?;
                self.skip_whitespace();
                if self.chars.next() == Some(')') {
                    Ok(val)
                } else {
                    Err("Mismatched opening parenthesis".into())
                }
            }
            Some(&c) if c.is_ascii_digit() || c == '.' => self.parse_number(),
            Some(&c) if c.is_alphabetic() => self.parse_identifier_or_function(),
            Some(&c) => Err(format!("Unexpected character '{}'", c)),
            None => Err("Unexpected end of expression".into()),
        }
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let mut s = String::new();
        let mut has_dot = false;
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.chars.next();
            } else if c == '.' && !has_dot {
                has_dot = true;
                s.push(c);
                self.chars.next();
            } else if (c == 'e' || c == 'E') && !s.is_empty() && self.starts_an_exponent() {
                s.push(c);
                self.chars.next();
                if let Some(&sign) = self.chars.peek() {
                    if sign == '+' || sign == '-' {
                        s.push(sign);
                        self.chars.next();
                    }
                }
                while let Some(&digit) = self.chars.peek() {
                    if !digit.is_ascii_digit() {
                        break;
                    }
                    s.push(digit);
                    self.chars.next();
                }
                break;
            } else {
                break;
            }
        }

        s.parse::<f64>()
            .map_err(|_| format!("Invalid number token '{}'", s))
    }

    /// Whether the `e` that comes next is `1e5`, not the constant in `2e`
    fn starts_an_exponent(&self) -> bool {
        let mut next = self.chars.clone();
        next.next();
        match next.next() {
            Some(c) if c.is_ascii_digit() => true,
            Some('+') | Some('-') => next.next().is_some_and(|c| c.is_ascii_digit()),
            _ => false,
        }
    }

    fn parse_identifier_or_function(&mut self) -> Result<f64, String> {
        let mut name = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                name.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        self.skip_whitespace();
        if let Some(&'(') = self.chars.peek() {
            // Function call
            self.chars.next();
            let arg = self.parse_expression()?;
            self.skip_whitespace();
            if self.chars.next() != Some(')') {
                return Err(format!("Expected closing ')' after function argument in {}", name));
            }

            let value = match name.as_str() {
                "abs" => arg.abs(),
                "sqrt" => arg.sqrt(),
                "cbrt" => arg.cbrt(),
                "floor" => arg.floor(),
                "ceil" => arg.ceil(),
                "round" => arg.round(),
                "sin" => arg.sin(),
                "cos" => arg.cos(),
                "tan" => arg.tan(),
                "asin" => arg.asin(),
                "acos" => arg.acos(),
                "atan" => arg.atan(),
                "ln" => arg.ln(),
                "log10" => arg.log10(),
                "log2" => arg.log2(),
                "exp" => arg.exp(),
                "log" => return Err("'log' is ambiguous: use log10 or ln".into()),
                _ => return Err(format!("Unknown math function '{}'", name)),
            };
            if value.is_nan() || (value.is_infinite() && matches!(name.as_str(), "ln" | "log10" | "log2")) {
                return Err(format!("{name}({arg}) is not a real number"));
            }
            Ok(value)
        } else {
            // Constants
            match name.as_str() {
                "pi" | "PI" => Ok(core::f64::consts::PI),
                "e" | "E" => Ok(core::f64::consts::E),
                _ => Err(format!("Unknown constant or variable '{}'", name)),
            }
        }
    }
}

pub fn evaluate_expression(expr: &str) -> Result<f64, String> {
    let mut parser = Parser::new(expr);
    let result = parser.parse_expression()?;
    parser.skip_whitespace();
    if parser.chars.peek().is_some() {
        return Err("Unexpected trailing characters in expression".into());
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::evaluate_expression as eval;

    fn close(expression: &str, expected: f64) {
        let got = eval(expression).unwrap();
        assert!((got - expected).abs() <= expected.abs() * 1e-14 + 1e-14, "{expression} gave {got}");
    }

    #[test]
    fn functions_are_accurate_far_from_zero() {
        close("ln(1000000)", 13.815510557964274);
        close("exp(50)", 5184705528587072045056.0);
        close("sin(1000)", 0.8268795405320025);
        close("log10(1000)", 3.0);
        close("2^0.5", 1.4142135623730951);
    }

    #[test]
    fn order_of_operations() {
        close("2 + 3 * 4", 14.0);
        close("-2^2", -4.0);
        close("2^-1", 0.5);
        close("2^3^2", 512.0);
        close("(1 + 2) * -3", -9.0);
        close("1.5e3 + 1", 1501.0);
        close("2 * e", 2.0 * core::f64::consts::E);
    }

    #[test]
    fn what_has_no_answer_is_an_error_not_a_number() {
        for expression in ["1/0", "sqrt(-1)", "ln(0)", "(-8)^0.5", "log(10)", "2 +", "foo", "1 2"] {
            assert!(eval(expression).is_err(), "{expression}");
        }
    }
}
