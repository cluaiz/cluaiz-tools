//! Unit conversion: each unit is a factor to its quantity's base unit; temperatures use
//! their formulas. Names are matched exactly first, then without case when only one unit fits.

pub struct Unit {
    pub name: &'static str,
    pub quantity: &'static str,
    factor: f64,
    names: &'static [&'static str],
}

const fn u(name: &'static str, quantity: &'static str, factor: f64, names: &'static [&'static str]) -> Unit {
    Unit { name, quantity, factor, names }
}

const INCH: f64 = 0.0254;
const FOOT: f64 = 0.3048;
const MILE: f64 = 1609.344;
const POUND: f64 = 0.45359237;
const US_GALLON: f64 = 3.785411784;
const IMPERIAL_GALLON: f64 = 4.54609;

static UNITS: &[Unit] = &[
    // Length, metres
    u("m", "length", 1.0, &["m", "meter", "meters", "metre", "metres"]),
    u("km", "length", 1000.0, &["km", "kilometer", "kilometers", "kilometre", "kilometres"]),
    u("cm", "length", 0.01, &["cm", "centimeter", "centimeters", "centimetre", "centimetres"]),
    u("mm", "length", 0.001, &["mm", "millimeter", "millimeters", "millimetre", "millimetres"]),
    u("um", "length", 1e-6, &["um", "µm", "micrometer", "micrometers", "micron", "microns"]),
    u("nm", "length", 1e-9, &["nm", "nanometer", "nanometers"]),
    u("in", "length", INCH, &["in", "inch", "inches", "\""]),
    u("ft", "length", FOOT, &["ft", "foot", "feet", "'"]),
    u("yd", "length", 0.9144, &["yd", "yard", "yards"]),
    u("mi", "length", MILE, &["mi", "mile", "miles"]),
    u("nmi", "length", 1852.0, &["nmi", "nautical mile", "nautical miles"]),
    u("au", "length", 149_597_870_700.0, &["au", "astronomical unit"]),
    u("ly", "length", 9_460_730_472_580_800.0, &["ly", "light year", "light years", "lightyear"]),
    // Mass, kilograms
    u("kg", "mass", 1.0, &["kg", "kilogram", "kilograms", "kilo", "kilos"]),
    u("g", "mass", 0.001, &["g", "gram", "grams", "gm"]),
    u("mg", "mass", 1e-6, &["mg", "milligram", "milligrams"]),
    u("ug", "mass", 1e-9, &["ug", "µg", "mcg", "microgram", "micrograms"]),
    u("t", "mass", 1000.0, &["t", "tonne", "tonnes", "metric ton", "metric tons"]),
    u("lb", "mass", POUND, &["lb", "lbs", "pound", "pounds"]),
    u("oz", "mass", POUND / 16.0, &["oz", "ounce", "ounces"]),
    u("st", "mass", POUND * 14.0, &["st", "stone", "stones"]),
    u("ton_us", "mass", POUND * 2000.0, &["ton_us", "short ton", "short tons", "us ton"]),
    u("ton_uk", "mass", POUND * 2240.0, &["ton_uk", "long ton", "long tons", "uk ton"]),
    // Time, seconds
    u("s", "time", 1.0, &["s", "sec", "secs", "second", "seconds"]),
    u("ms", "time", 0.001, &["ms", "millisecond", "milliseconds"]),
    u("us", "time", 1e-6, &["us", "µs", "microsecond", "microseconds"]),
    u("ns", "time", 1e-9, &["ns", "nanosecond", "nanoseconds"]),
    u("min", "time", 60.0, &["min", "mins", "minute", "minutes"]),
    u("h", "time", 3600.0, &["h", "hr", "hrs", "hour", "hours"]),
    u("day", "time", 86_400.0, &["d", "day", "days"]),
    u("week", "time", 604_800.0, &["wk", "week", "weeks"]),
    u("year", "time", 31_557_600.0, &["yr", "year", "years"]),
    // Area, square metres
    u("m2", "area", 1.0, &["m2", "m^2", "sq m", "square meter", "square meters", "square metre", "square metres"]),
    u("km2", "area", 1e6, &["km2", "km^2", "sq km", "square kilometer", "square kilometers"]),
    u("cm2", "area", 1e-4, &["cm2", "cm^2", "sq cm", "square centimeter", "square centimeters"]),
    u("ha", "area", 1e4, &["ha", "hectare", "hectares"]),
    u("acre", "area", 4046.8564224, &["acre", "acres", "ac"]),
    u("ft2", "area", FOOT * FOOT, &["ft2", "ft^2", "sq ft", "sqft", "square foot", "square feet"]),
    u("in2", "area", INCH * INCH, &["in2", "in^2", "sq in", "square inch", "square inches"]),
    u("yd2", "area", 0.9144 * 0.9144, &["yd2", "yd^2", "sq yd", "square yard", "square yards"]),
    u("mi2", "area", MILE * MILE, &["mi2", "mi^2", "sq mi", "square mile", "square miles"]),
    // Volume, litres
    u("l", "volume", 1.0, &["l", "L", "liter", "liters", "litre", "litres"]),
    u("ml", "volume", 0.001, &["ml", "mL", "milliliter", "milliliters", "millilitre", "millilitres"]),
    u("m3", "volume", 1000.0, &["m3", "m^3", "cubic meter", "cubic meters", "cubic metre", "cubic metres"]),
    u("cm3", "volume", 0.001, &["cm3", "cm^3", "cc", "cubic centimeter", "cubic centimeters"]),
    u("ft3", "volume", FOOT * FOOT * FOOT * 1000.0, &["ft3", "ft^3", "cubic foot", "cubic feet"]),
    u("gal", "volume", US_GALLON, &["gal", "gallon", "gallons", "us gallon", "us gallons"]),
    u("qt", "volume", US_GALLON / 4.0, &["qt", "quart", "quarts"]),
    u("pt", "volume", US_GALLON / 8.0, &["pt", "pint", "pints"]),
    u("cup", "volume", US_GALLON / 16.0, &["cup", "cups"]),
    u("floz", "volume", US_GALLON / 128.0, &["floz", "fl oz", "fluid ounce", "fluid ounces"]),
    u("tbsp", "volume", US_GALLON / 256.0, &["tbsp", "tablespoon", "tablespoons"]),
    u("tsp", "volume", US_GALLON / 768.0, &["tsp", "teaspoon", "teaspoons"]),
    u("imp_gal", "volume", IMPERIAL_GALLON, &["imp_gal", "imperial gallon", "imperial gallons", "uk gallon", "uk gallons"]),
    u("imp_pt", "volume", IMPERIAL_GALLON / 8.0, &["imp_pt", "imperial pint", "imperial pints", "uk pint", "uk pints"]),
    // Speed, metres per second
    u("m/s", "speed", 1.0, &["m/s", "mps", "meters per second", "metres per second"]),
    u("km/h", "speed", 1000.0 / 3600.0, &["km/h", "kmh", "kph", "kmph", "kilometers per hour", "kilometres per hour"]),
    u("mph", "speed", MILE / 3600.0, &["mph", "mi/h", "miles per hour"]),
    u("kn", "speed", 1852.0 / 3600.0, &["kn", "kt", "knot", "knots"]),
    u("ft/s", "speed", FOOT, &["ft/s", "fps", "feet per second"]),
    // Data, bytes
    u("bit", "data", 0.125, &["bit", "bits", "b"]),
    u("B", "data", 1.0, &["B", "byte", "bytes"]),
    u("kB", "data", 1e3, &["kB", "KB", "kilobyte", "kilobytes"]),
    u("MB", "data", 1e6, &["MB", "megabyte", "megabytes"]),
    u("GB", "data", 1e9, &["GB", "gigabyte", "gigabytes"]),
    u("TB", "data", 1e12, &["TB", "terabyte", "terabytes"]),
    u("PB", "data", 1e15, &["PB", "petabyte", "petabytes"]),
    u("KiB", "data", 1024.0, &["KiB", "kibibyte", "kibibytes"]),
    u("MiB", "data", 1_048_576.0, &["MiB", "mebibyte", "mebibytes"]),
    u("GiB", "data", 1_073_741_824.0, &["GiB", "gibibyte", "gibibytes"]),
    u("TiB", "data", 1_099_511_627_776.0, &["TiB", "tebibyte", "tebibytes"]),
    u("kbit", "data", 125.0, &["kbit", "kb", "Kb", "kilobit", "kilobits"]),
    u("Mbit", "data", 125_000.0, &["Mbit", "Mb", "megabit", "megabits"]),
    u("Gbit", "data", 125_000_000.0, &["Gbit", "Gb", "gigabit", "gigabits"]),
    // Energy, joules
    u("J", "energy", 1.0, &["J", "joule", "joules"]),
    u("kJ", "energy", 1e3, &["kJ", "kilojoule", "kilojoules"]),
    u("cal", "energy", 4.184, &["cal", "calorie", "calories"]),
    u("kcal", "energy", 4184.0, &["kcal", "Cal", "kilocalorie", "kilocalories"]),
    u("Wh", "energy", 3600.0, &["Wh", "watt hour", "watt hours"]),
    u("kWh", "energy", 3.6e6, &["kWh", "kilowatt hour", "kilowatt hours", "unit", "units"]),
    u("eV", "energy", 1.602176634e-19, &["eV", "electronvolt", "electronvolts"]),
    u("BTU", "energy", 1055.05585262, &["BTU", "btu"]),
    // Power, watts
    u("W", "power", 1.0, &["W", "watt", "watts"]),
    u("kW", "power", 1e3, &["kW", "kilowatt", "kilowatts"]),
    u("MW", "power", 1e6, &["MW", "megawatt", "megawatts"]),
    u("hp", "power", 745.69987158227022, &["hp", "horsepower"]),
    // Pressure, pascals
    u("Pa", "pressure", 1.0, &["Pa", "pascal", "pascals"]),
    u("kPa", "pressure", 1e3, &["kPa", "kilopascal", "kilopascals"]),
    u("bar", "pressure", 1e5, &["bar", "bars"]),
    u("atm", "pressure", 101_325.0, &["atm", "atmosphere", "atmospheres"]),
    u("psi", "pressure", 6894.757293168361, &["psi"]),
    u("mmHg", "pressure", 133.322387415, &["mmHg", "torr"]),
    // Angle, radians
    u("rad", "angle", 1.0, &["rad", "radian", "radians"]),
    u("deg", "angle", core::f64::consts::PI / 180.0, &["deg", "degree", "degrees", "°"]),
    u("grad", "angle", core::f64::consts::PI / 200.0, &["grad", "gon", "gradian", "gradians"]),
    u("turn", "angle", core::f64::consts::TAU, &["turn", "turns", "rev", "revolution", "revolutions"]),
    // Temperature: converted by formula in `convert`, so the factor is not used
    u("C", "temperature", 1.0, &["C", "°C", "celsius", "centigrade", "degC"]),
    u("F", "temperature", 1.0, &["F", "°F", "fahrenheit", "degF"]),
    u("K", "temperature", 1.0, &["K", "kelvin", "kelvins"]),
];

/// The unit a name means: an exact match, else the only unit matching without case
pub fn find(name: &str) -> Result<&'static Unit, String> {
    let name = name.trim();
    if let Some(unit) = UNITS.iter().find(|unit| unit.names.contains(&name)) {
        return Ok(unit);
    }
    let lower = name.to_lowercase();
    let mut matches = UNITS.iter().filter(|unit| unit.names.iter().any(|n| n.to_lowercase() == lower));
    match (matches.next(), matches.next()) {
        (Some(unit), None) => Ok(unit),
        (Some(a), Some(b)) => Err(format!("'{name}' could mean {} or {}; write the unit with its exact case", a.name, b.name)),
        _ => Err(format!("Unknown unit '{name}'. Known quantities: {}", quantities().join(", "))),
    }
}

fn quantities() -> Vec<&'static str> {
    let mut list: Vec<&str> = Vec::new();
    for unit in UNITS {
        if !list.contains(&unit.quantity) {
            list.push(unit.quantity);
        }
    }
    list
}

/// `value` in `from` expressed in `to`
pub fn convert(value: f64, from: &str, to: &str) -> Result<(f64, &'static Unit, &'static Unit), String> {
    let (from, to) = (find(from)?, find(to)?);
    if from.quantity != to.quantity {
        return Err(format!("Cannot convert {} ({}) to {} ({})", from.name, from.quantity, to.name, to.quantity));
    }
    if from.quantity == "temperature" {
        // Through Celsius with the usual formulas, which keep 100 C = 212 F exact
        let celsius = match from.name {
            "F" => (value - 32.0) * 5.0 / 9.0,
            "K" => value - 273.15,
            _ => value,
        };
        if celsius < -273.15 {
            return Err(format!("{value} {} is below absolute zero", from.name));
        }
        let result = match to.name {
            "F" => celsius * 9.0 / 5.0 + 32.0,
            "K" => celsius + 273.15,
            _ => celsius,
        };
        return Ok((tidy(result), from, to));
    }
    Ok((tidy(value * from.factor / to.factor), from, to))
}

/// Rounds to 15 significant digits, which removes the float noise of the factors
/// (0.30000000000000004 becomes 0.3) and keeps every digit a 64-bit float can promise
fn tidy(value: f64) -> f64 {
    if value == 0.0 || !value.is_finite() {
        return value;
    }
    format!("{:.14e}", value).parse().unwrap_or(value)
}

#[cfg(test)]
mod tests {
    use super::convert;

    #[test]
    fn results_are_exact_where_the_numbers_are() {
        assert_eq!(convert(100.0, "C", "F").unwrap().0, 212.0);
        assert_eq!(convert(98.6, "F", "C").unwrap().0, 37.0);
        assert_eq!(convert(1.0, "ft", "in").unwrap().0, 12.0);
        assert_eq!(convert(3.0, "km", "m").unwrap().0, 3000.0);
    }

    fn close(value: f64, from: &str, to: &str, expected: f64) {
        let (got, _, _) = convert(value, from, to).unwrap();
        assert!((got - expected).abs() <= expected.abs() * 1e-12 + 1e-12, "{value} {from} -> {to} gave {got}");
    }

    #[test]
    fn units_convert_exactly() {
        close(1.0, "mi", "km", 1.609344);
        close(100.0, "C", "F", 212.0);
        close(-40.0, "F", "C", -40.0);
        close(0.0, "K", "C", -273.15);
        close(1.0, "GiB", "MB", 1073.741824);
        close(1.0, "Mb", "kB", 125.0);
        close(1.0, "kWh", "J", 3.6e6);
        close(180.0, "degrees", "rad", core::f64::consts::PI);
        close(1.0, "gallon", "litres", 3.785411784);
        close(60.0, "mph", "km/h", 96.56064);
        close(2.0, "Square Feet", "m2", 0.18580608);
    }

    #[test]
    fn no_name_means_two_units() {
        let mut seen = std::collections::HashSet::new();
        for unit in super::UNITS {
            for name in unit.names {
                assert!(seen.insert(*name), "'{name}' is listed twice");
            }
        }
    }

    #[test]
    fn wrong_units_are_errors() {
        assert!(convert(1.0, "kg", "m").is_err());
        assert!(convert(1.0, "mb", "kB").is_err(), "mb could be MB or Mb");
        assert!(convert(1.0, "parsec", "m").is_err());
        assert!(convert(-300.0, "C", "K").is_err());
    }
}
