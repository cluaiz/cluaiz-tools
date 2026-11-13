use serde::{Deserialize, Serialize};

mod units;

/// `math_evaluate` sends `expression`; `math_convert` sends `value`, `from` and `to`.
/// The Engine passes only the arguments, so they tell the two apart.
#[derive(Deserialize)]
struct MathRequest {
    #[serde(default)]
    expression: Option<String>,
    #[serde(default)]
    precision: Option<usize>,
    #[serde(default)]
    value: Option<f64>,
    #[serde(default)]
    from: Option<String>,
    #[serde(default)]
    to: Option<String>,
}

#[derive(Serialize)]
struct ConvertResponse {
    status: &'static str,
    value: f64,
    from: &'static str,
    to: &'static str,
    quantity: &'static str,
    result: f64,
    formatted: String,
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

    let output = match serde_json::from_slice::<MathRequest>(input_bytes) {
        Ok(MathRequest { value: Some(value), from: Some(from), to: Some(to), precision, .. }) => {
            match units::convert(value, &from, &to) {
                Ok((result, from, to)) => serde_json::to_vec(&ConvertResponse {
                    status: "success",
                    value,
                    from: from.name,
                    to: to.name,
                    quantity: from.quantity,
                    result,
                    formatted: format_number(result, precision),
                }),
                Err(error) => serde_json::to_vec(&serde_json::json!({"status": "error", "error": error})),
            }
        }
        Ok(MathRequest { expression: Some(expression), precision, .. }) => serde_json::to_vec(&match evaluate_expression(&expression) {
            Ok(val) => MathResponse {
                status: "success".into(),
                expression,
                result: Some(val),
                formatted: Some(format_number(val, precision)),
                note: (val.abs() > LARGEST_EXACT)
                    .then(|| "This result is too large for a 64-bit float to hold exactly".to_string()),
                error: None,
            },
            Err(e) => failure(expression, e),
        }),
        _ => serde_json::to_vec(&failure(
            String::new(),
            "Expected {\"expression\": \"...\"} or {\"value\": 1, \"from\": \"km\", \"to\": \"mi\"}".into(),
        )),
    }
    .unwrap_or_else(|_| Vec::from(b"{\"status\":\"error\",\"error\":\"Serialization failure\"}".as_slice()));

    let out_len = output.len() as u64;
    let out_ptr = output.as_ptr() as u64;
    core::mem::forget(output);

    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}

fn failure(expression: String, error: String) -> MathResponse {
    MathResponse { status: "error".into(), expression, result: None, formatted: None, note: None, error: Some(error) }
}

fn format_number(value: f64, precision: Option<usize>) -> String {
    match precision {
        Some(p) => format!("{:.prec$}", value, prec = p),
        None => format!("{}", value),
    }
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
        let mut left = self.parse_factor()?;
        self.skip_whitespace();
        while self.chars.peek() == Some(&'!') {
            self.chars.next();
            left = factorial(left)?;
            self.skip_whitespace();
        }
        let mut ahead = self.chars.clone();
        let first = ahead.next();
        let python_power = first == Some('*') && ahead.next() == Some('*');
        if first == Some('^') || python_power {
            self.chars.next();
            if python_power {
                self.chars.next();
            }
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
            self.chars.next();
            let args = self.parse_arguments(&name)?;
            call_function(&name, &args)
        } else {
            match name.as_str() {
                "pi" | "PI" => Ok(core::f64::consts::PI),
                "e" | "E" => Ok(core::f64::consts::E),
                _ => Err(format!("Unknown constant or variable '{}'", name)),
            }
        }
    }

    /// The comma-separated arguments of a function, after its `(`, up to and including the `)`
    fn parse_arguments(&mut self, name: &str) -> Result<Vec<f64>, String> {
        let mut args = vec![self.parse_expression()?];
        loop {
            self.skip_whitespace();
            match self.chars.next() {
                Some(')') => return Ok(args),
                Some(',') => args.push(self.parse_expression()?),
                _ => return Err(format!("Expected ',' or ')' in {name}(...)")),
            }
        }
    }
}

fn call_function(name: &str, args: &[f64]) -> Result<f64, String> {
    let one = |f: fn(f64) -> f64| -> Result<f64, String> {
        match args {
            [x] => Ok(f(*x)),
            _ => Err(format!("{name} takes one argument, not {}", args.len())),
        }
    };
    let value = match name {
        "abs" => one(f64::abs)?,
        "sqrt" => one(f64::sqrt)?,
        "cbrt" => one(f64::cbrt)?,
        "floor" => one(f64::floor)?,
        "ceil" => one(f64::ceil)?,
        "round" => one(f64::round)?,
        "sin" => one(f64::sin)?,
        "cos" => one(f64::cos)?,
        "tan" => one(f64::tan)?,
        "asin" => one(f64::asin)?,
        "acos" => one(f64::acos)?,
        "atan" => one(f64::atan)?,
        "ln" => one(f64::ln)?,
        "log10" => one(f64::log10)?,
        "log2" => one(f64::log2)?,
        "exp" => one(f64::exp)?,
        "log" => match args {
            [x, base] if *base > 0.0 && *base != 1.0 => x.ln() / base.ln(),
            [_, base] => return Err(format!("log base {base} is not valid: it must be above 0 and not 1")),
            _ => return Err("'log' needs a base: log(x, base). Use log10(x) or ln(x) otherwise".into()),
        },
        "min" => args.iter().copied().fold(f64::INFINITY, f64::min),
        "max" => args.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        "factorial" | "fact" => match args {
            [n] => factorial(*n)?,
            _ => return Err(format!("{name} takes one argument")),
        },
        "nCr" | "ncr" | "comb" => match args {
            [n, r] => choose(*n, *r, true)?,
            _ => return Err(format!("{name} takes two arguments: {name}(n, r)")),
        },
        "nPr" | "npr" | "perm" => match args {
            [n, r] => choose(*n, *r, false)?,
            _ => return Err(format!("{name} takes two arguments: {name}(n, r)")),
        },
        _ => return Err(format!("Unknown math function '{}'", name)),
    };
    let shown = args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
    if value.is_nan() || (value.is_infinite() && matches!(name, "ln" | "log10" | "log2" | "log")) {
        return Err(format!("{name}({shown}) is not a real number"));
    }
    if value.is_infinite() && args.iter().all(|a| a.is_finite()) {
        return Err(format!("{name}({shown}) is too large for a 64-bit float"));
    }
    Ok(value)
}

/// A whole number that is not negative, or an error naming what it is for
fn whole(value: f64, what: &str) -> Result<u64, String> {
    if value < 0.0 || value.fract() != 0.0 || value > LARGEST_EXACT {
        return Err(format!("{what} must be a whole number of 0 or more, not {value}"));
    }
    Ok(value as u64)
}

fn factorial(n: f64) -> Result<f64, String> {
    let n = whole(n, "factorial")?;
    if n > 170 {
        return Err(format!("{n}! is too large for a 64-bit float (the largest is 170!)"));
    }
    Ok((2..=n).fold(1.0, |product, k| product * k as f64))
}

/// Combinations (`combine`) or permutations of `r` items out of `n`
fn choose(n: f64, r: f64, combine: bool) -> Result<f64, String> {
    let (n, r) = (whole(n, "n")?, whole(r, "r")?);
    if r > n {
        return Ok(0.0);
    }
    if combine {
        let r = r.min(n - r);
        // Multiplying and dividing in turn keeps every step a whole number
        Ok((1..=r).fold(1.0, |acc, k| acc * (n - r + k) as f64 / k as f64).round())
    } else {
        Ok((n - r + 1..=n).fold(1.0, |acc, k| acc * k as f64))
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
    fn functions_with_several_arguments() {
        close("log(8, 2)", 3.0);
        close("log(1000, 10)", 3.0);
        close("min(4, -2, 7)", -2.0);
        close("max(4, -2, 7)", 7.0);
        close("nCr(52, 5)", 2598960.0);
        close("nPr(10, 3)", 720.0);
        close("nCr(3, 5)", 0.0);
        close("factorial(5)", 120.0);
        close("5!", 120.0);
        close("3!^2", 36.0);
        close("-3!", -6.0);
        close("0!", 1.0);
        close("max(1, 2) + min(3, 4)", 5.0);
    }

    #[test]
    fn what_has_no_answer_is_an_error_not_a_number() {
        for expression in ["1/0", "sqrt(-1)", "ln(0)", "(-8)^0.5", "log(10)", "log(8, 1)", "2 +", "foo", "1 2", "171!", "2.5!", "nCr(5)", "sqrt(1, 2)", "nCr(-1, 2)"] {
            assert!(eval(expression).is_err(), "{expression}");
        }
    }
}
