//! Exact resource quantities from the pinned apimachinery quantity/suffix/amount rules.
//!
//! Decimal exponents stay factored: comparison never expands `10^exponent` or
//! passes through a float. BigUint supplies the binary suffix multiplication.
use std::{cmp::Ordering, collections::BTreeMap, fmt, str::FromStr};

use cgp::prelude::*;
use num_bigint::BigUint;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{ValidationError, ValidationErrorKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Format {
    Unset,
    DecimalSi,
    BinarySi,
    DecimalExponent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseQuantityError {
    Format,
    Numeric,
    Suffix,
}

impl fmt::Display for ParseQuantityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Format => "quantities must match the regular expression '^([+-]?[0-9.]+)([eEinumkKMGTP]*[-+]?[0-9]*)$'",
            Self::Numeric => "unable to parse numeric part of quantity",
            Self::Suffix => "unable to parse quantity's suffix",
        })
    }
}
impl std::error::Error for ParseQuantityError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceQuantity {
    // Unsigned coefficient, with leading zeros removed. Zero is always "0".
    digits: String,
    exponent: i32,
    negative: bool,
    format: Format,
    cached: Option<String>,
    fast: bool,
}

impl Default for ResourceQuantity {
    fn default() -> Self {
        Self::from_fast("0".into(), 0, false, Format::Unset, None)
    }
}

impl ResourceQuantity {
    fn new(
        digits: String,
        exponent: i32,
        negative: bool,
        format: Format,
        cached: Option<String>,
    ) -> Self {
        let digits = digits.trim_start_matches('0');
        let zero = digits.is_empty();
        Self {
            digits: if zero { "0".into() } else { digits.into() },
            exponent,
            negative: negative && !zero,
            format,
            cached,
            fast: false,
        }
    }

    fn from_fast(
        digits: String,
        exponent: i32,
        negative: bool,
        format: Format,
        cached: Option<String>,
    ) -> Self {
        let mut value = Self::new(digits, exponent, negative, format, cached);
        value.fast = true;
        value
    }

    pub fn is_negative(&self) -> bool {
        self.negative
    }
    pub fn is_zero(&self) -> bool {
        self.digits == "0"
    }

    fn compare_magnitude(&self, other: &Self) -> Ordering {
        match (self.is_zero(), other.is_zero()) {
            (true, true) => return Ordering::Equal,
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => {}
        }
        let order = (self.digits.len() as i64 + i64::from(self.exponent))
            .cmp(&(other.digits.len() as i64 + i64::from(other.exponent)));
        if order != Ordering::Equal {
            return order;
        }
        // Equal decimal magnitude: compare coefficients with virtual zero padding.
        let width = self.digits.len().max(other.digits.len());
        self.digits
            .bytes()
            .chain(std::iter::repeat(b'0'))
            .take(width)
            .cmp(
                other
                    .digits
                    .bytes()
                    .chain(std::iter::repeat(b'0'))
                    .take(width),
            )
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        // Preserve the pinned int64Amount.Cmp exceptional branch. It subtracts
        // int32 scales before comparing signs; overflow reaches a zero divisor.
        if self.fast
            && other.fast
            && self
                .exponent
                .max(other.exponent)
                .wrapping_sub(self.exponent.min(other.exponent))
                < 0
        {
            panic!("runtime error: integer divide by zero");
        }
        match (self.negative, other.negative) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (true, true) => self.compare_magnitude(other).reverse(),
            (false, false) => self.compare_magnitude(other),
        }
    }

    fn round_to_nano(&mut self) {
        if self.is_zero() || self.exponent >= -9 {
            return;
        }
        let removed = (-9_i64 - i64::from(self.exponent)) as usize;
        let kept = self.digits.len().saturating_sub(removed);
        let round_up = self.digits[kept..].bytes().any(|byte| byte != b'0');
        let integer = BigUint::parse_bytes(self.digits[..kept].as_bytes(), 10).unwrap_or_default();
        self.digits = (integer + BigUint::from(u8::from(round_up))).to_str_radix(10);
        self.exponent = -9;
    }

    fn integer_digits(&self) -> Option<String> {
        if self.exponent >= 0 {
            let mut digits = self.digits.clone();
            digits.extend(std::iter::repeat_n('0', self.exponent as usize));
            Some(digits)
        } else {
            let removed = -i64::from(self.exponent) as usize;
            let kept = self.digits.len().checked_sub(removed)?;
            self.digits[kept..]
                .bytes()
                .all(|byte| byte == b'0')
                .then(|| self.digits[..kept].to_owned())
        }
    }

    fn decimal_string(&self, format: Format) -> String {
        let digits = self.digits.trim_end_matches('0');
        let mut exponent = self
            .exponent
            .wrapping_add((self.digits.len() - digits.len()) as i32);
        let padding = exponent.rem_euclid(3);
        let mut amount = digits.to_owned();
        amount.extend(std::iter::repeat_n('0', padding as usize));
        exponent = exponent.wrapping_sub(padding);
        let suffix = match format {
            Format::DecimalExponent if exponent != 0 => format!("e{exponent}"),
            Format::DecimalExponent => String::new(),
            // The source constructor returns an empty suffix outside this table.
            _ => match exponent {
                -9 => "n",
                -6 => "u",
                -3 => "m",
                0 => "",
                3 => "k",
                6 => "M",
                9 => "G",
                12 => "T",
                15 => "P",
                18 => "E",
                _ => "",
            }
            .to_owned(),
        };
        format!("{}{amount}{suffix}", if self.negative { "-" } else { "" })
    }
}

fn suffix(suffix: &str) -> Result<(bool, i32, Format), ParseQuantityError> {
    let decimal = match suffix {
        "n" => Some(-9),
        "u" => Some(-6),
        "m" => Some(-3),
        "" => Some(0),
        "k" => Some(3),
        "M" => Some(6),
        "G" => Some(9),
        "T" => Some(12),
        "P" => Some(15),
        "E" => Some(18),
        _ => None,
    };
    if let Some(exponent) = decimal {
        return Ok((false, exponent, Format::DecimalSi));
    }
    if let Some(index) = ["Ki", "Mi", "Gi", "Ti", "Pi", "Ei"]
        .iter()
        .position(|s| *s == suffix)
    {
        return Ok((true, (index as i32 + 1) * 10, Format::BinarySi));
    }
    if suffix.starts_with(['e', 'E']) && suffix.len() > 1 {
        let exponent = suffix[1..]
            .parse::<i64>()
            .map_err(|_| ParseQuantityError::Suffix)?;
        // The pinned suffix interpreter explicitly narrows an int64 to int32.
        return Ok((false, exponent as i32, Format::DecimalExponent));
    }
    Err(ParseQuantityError::Suffix)
}

impl FromStr for ResourceQuantity {
    type Err = ParseQuantityError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(ParseQuantityError::Format);
        }
        if input == "0" {
            return Ok(Self::from_fast(
                "0".into(),
                0,
                false,
                Format::DecimalSi,
                Some("0".into()),
            ));
        }
        let bytes = input.as_bytes();
        let negative = bytes[0] == b'-';
        let mut pos = usize::from(matches!(bytes[0], b'+' | b'-'));
        while pos < bytes.len() && bytes[pos] == b'0' {
            pos += 1;
        }
        // The source scanner accepts sign-only and all-zero inputs as zero.
        if pos == bytes.len() {
            return Ok(Self::from_fast(
                "0".into(),
                0,
                false,
                Format::DecimalSi,
                None,
            ));
        }
        let start = pos;
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
        let num = if pos == start {
            "0"
        } else {
            &input[start..pos]
        };
        let mut denom = "";
        if pos < bytes.len() && bytes[pos] == b'.' {
            pos += 1;
            let start = pos;
            while pos < bytes.len() && bytes[pos].is_ascii_digit() {
                pos += 1;
            }
            denom = &input[start..pos];
        }
        let value = &input[..pos];
        let suffix_start = pos;
        while pos < bytes.len() && b"eEinumkKMGTP".contains(&bytes[pos]) {
            pos += 1;
        }
        if pos < bytes.len() && matches!(bytes[pos], b'+' | b'-') {
            pos += 1;
        }
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
        if pos != bytes.len() {
            return Err(ParseQuantityError::Format);
        }
        let (binary, exponent, format) = suffix(&input[suffix_start..])?;
        let scale = if binary { 0 } else { exponent }.wrapping_sub(denom.len() as i32);
        let precision = if binary {
            if denom.is_empty() {
                14 - num.len() as i32 - exponent * 3 / 10
            } else {
                -1
            }
        } else {
            18 - (num.len() + denom.len()) as i32
        };
        let shifted = format!("{num}{denom}");
        if precision >= 0 && scale >= -9 {
            let coefficient = shifted
                .parse::<i64>()
                .map_err(|_| ParseQuantityError::Numeric)?;
            let multiplier = if binary { 1_i64 << exponent } else { 1 };
            if let Some(result) = coefficient.checked_mul(multiplier) {
                let cache = if binary {
                    coefficient & 7 != 0
                } else {
                    scale % 3 == 0 && !shifted.ends_with("000") && shifted.as_bytes()[0] != b'0'
                };
                return Ok(Self::from_fast(
                    result.to_string(),
                    scale,
                    negative,
                    format,
                    cache.then(|| input.to_owned()),
                ));
            }
        }
        // inf.Dec.SetString rejects a missing numeric value in the slow path.
        if !value.bytes().any(|byte| byte.is_ascii_digit()) {
            return Err(ParseQuantityError::Numeric);
        }
        let mut coefficient =
            BigUint::parse_bytes(shifted.as_bytes(), 10).ok_or(ParseQuantityError::Numeric)?;
        if binary {
            coefficient <<= exponent as usize;
        }
        let mut quantity = Self::new(coefficient.to_str_radix(10), scale, negative, format, None);
        quantity.round_to_nano();
        if binary {
            let maximum = Self::new(i64::MAX.to_string(), 0, false, format, None);
            if quantity.compare_magnitude(&maximum) == Ordering::Greater {
                quantity.digits = maximum.digits;
                quantity.exponent = 0;
            }
            if !quantity.is_zero()
                && quantity.compare_magnitude(&Self::new("1".into(), 0, false, format, None))
                    == Ordering::Less
            {
                quantity.format = Format::DecimalSi;
            }
        }
        Ok(quantity)
    }
}

impl fmt::Display for ResourceQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(cached) = &self.cached {
            return f.write_str(cached);
        }
        if self.is_zero() {
            return f.write_str("0");
        }
        if self.format == Format::BinarySi
            && self.compare_magnitude(&Self::new("1024".into(), 0, false, Format::DecimalSi, None))
                != Ordering::Less
            && let Some(integer) = self.integer_digits()
        {
            let mut amount = integer
                .parse::<u64>()
                .expect("binary quantities are capped at int64");
            let mut exponent = 0;
            while amount % 1024 == 0 {
                amount /= 1024;
                exponent += 1;
            }
            let suffix = ["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei"][exponent];
            return write!(
                f,
                "{}{amount}{suffix}",
                if self.negative { "-" } else { "" }
            );
        }
        f.write_str(&self.decimal_string(if self.format == Format::BinarySi {
            Format::DecimalSi
        } else {
            self.format
        }))
    }
}

impl Serialize for ResourceQuantity {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> Deserialize<'de> for ResourceQuantity {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(deserializer)?;
        let text = raw.get();
        if text == "null" {
            return Ok(Self::default());
        }
        // Match Quantity.UnmarshalJSON: strip quotes, without unescaping them.
        let text = text
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(text);
        text.trim().parse().map_err(serde::de::Error::custom)
    }
}

/// Quantities remain typed through validation; String is an observation, not
/// storage. In particular the source renders 1000E as "1" while retaining 1e21.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerResourceRequirements {
    #[serde(default, skip_serializing_if = "quantity_map_is_empty")]
    pub limits: Option<BTreeMap<String, ResourceQuantity>>,
    #[serde(default, skip_serializing_if = "quantity_map_is_empty")]
    pub requests: Option<BTreeMap<String, ResourceQuantity>>,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

fn quantity_map_is_empty(map: &Option<BTreeMap<String, ResourceQuantity>>) -> bool {
    map.as_ref().is_none_or(BTreeMap::is_empty)
}

impl ResourceQuantity {
    pub(crate) fn semantic_key(&self) -> serde_json::Value {
        let digits = self.digits.trim_end_matches('0');
        let (digits, exponent) = if self.is_zero() {
            ("0".to_owned(), 0)
        } else {
            (
                format!("{}{digits}", if self.negative { "-" } else { "" }),
                i64::from(self.exponent) + (self.digits.len() - digits.len()) as i64,
            )
        };
        serde_json::json!({"$agonesQuantityCoefficient": digits, "$agonesQuantityExponent": exponent})
    }
}

#[cgp_component(ResourceQuantityValidator)]
pub trait CanValidateResourceQuantities {
    fn validate_resource_quantities(
        &self,
        limits: &BTreeMap<String, ResourceQuantity>,
        requests: &BTreeMap<String, ResourceQuantity>,
        path: &str,
    ) -> Vec<ValidationError>;
}

#[cgp_new_provider]
impl<Context> ResourceQuantityValidator<Context> for ValidateAgonesResourceQuantities {
    fn validate_resource_quantities(
        _context: &Context,
        limits: &BTreeMap<String, ResourceQuantity>,
        requests: &BTreeMap<String, ResourceQuantity>,
        path: &str,
    ) -> Vec<ValidationError> {
        let invalid = |path: String, value: &ResourceQuantity, detail: String| ValidationError {
            origin: String::new(),
            covered_by_declarative: false,
            kind: ValidationErrorKind::Invalid,
            path,
            bad_value: Some(value.to_string()),
            detail,
        };
        let mut errors = Vec::new();
        // Go map iteration is unspecified. Sort within each source phase.
        for (name, value) in limits {
            if value.is_negative() {
                errors.push(invalid(
                    format!("{path}.limits[{name}]"),
                    value,
                    "must be greater than or equal to 0".into(),
                ));
            }
        }
        for (name, value) in requests {
            if value.is_negative() {
                errors.push(invalid(
                    format!("{path}.requests[{name}]"),
                    value,
                    "must be greater than or equal to 0".into(),
                ));
            }
            if let Some(limit) = limits.get(name)
                && value.compare(limit) == Ordering::Greater
            {
                errors.push(invalid(
                    format!("{path}.requests"),
                    value,
                    format!("must be less than or equal to {name} limit of {limit}"),
                ));
            }
        }
        errors
    }
}

impl crate::GameServerSpec {
    /// Shared source-ordered resource traversal, independent of runtime DTOs.
    pub fn validate_resource_quantities_with_context<C: CanValidateResourceQuantities>(
        &self,
        context: &C,
        spec_path: &str,
    ) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let empty = BTreeMap::new();
        for (kind, containers) in [
            (
                "initContainers",
                self.template.spec.init_containers.as_slice(),
            ),
            ("containers", self.template.spec.containers.as_slice()),
        ] {
            for (index, container) in containers.iter().enumerate() {
                errors.extend(context.validate_resource_quantities(
                    container.resources.limits.as_ref().unwrap_or(&empty),
                    container.resources.requests.as_ref().unwrap_or(&empty),
                    &format!("{spec_path}.template.spec.{kind}[{index}].resources"),
                ));
            }
        }
        errors
    }
}
