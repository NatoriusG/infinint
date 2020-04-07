use std::fmt;

pub struct Infinint {
    negative: bool,
    digits: Vec<u8>,
}

impl Infinint {
    pub fn new() -> Infinint {
        Infinint {
            negative: false,
            digits: vec![0],
        }
    }
}

impl fmt::Debug for Infinint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_out = String::new();
        debug_out.push_str(format!("\nnegative: {}\n", self.negative).as_str());
        debug_out.push_str(format!("digits: [\n").as_str());
        for d in &self.digits {
            debug_out.push_str(
                format!(
                    "    {:04b}_{:04b} -> ({}, {})\n",
                    (0xF0 & *d) >> 4,
                    0xF & *d,
                    decimal_digit_high(*d).unwrap(),
                    decimal_digit_low(*d).unwrap()
                )
                .as_str(),
            );
        }
        debug_out.push_str("]\n");
        write!(f, "{}", debug_out)
    }
}

impl From<i32> for Infinint {
    fn from(n: i32) -> Self {
        let negative = n < 0;

        let bytes_needed = match n {
            0 => 1,
            _ => (((n as f64).abs().log10()) as usize / 2) + 1,
        };
        let mut digits: Vec<u8> = Vec::with_capacity(bytes_needed);

        let mut n = if n >= 0 { n } else { -n };

        while n > 0 {
            let mut d: u8;

            let n_mod = n % 10;
            d = (n_mod << 4) as u8;
            n /= 10;

            let n_mod = n % 10;
            d = d | n_mod as u8;
            n /= 10;

            digits.push(d);
        }

        Infinint { negative, digits }
    }
}

fn decimal_digits(n: u8) -> Result<(u8, u8), &'static str> {
    let high = decimal_digit_high(n)?;
    let low = decimal_digit_low(n)?;
    Ok((high, low))
}

fn decimal_digit_high(n: u8) -> Result<u8, &'static str> {
    decimal_digit_nybble((0xF0 & n) >> 4)
}

fn decimal_digit_low(n: u8) -> Result<u8, &'static str> {
    decimal_digit_nybble(0x0F & n)
}

fn decimal_digit_nybble(n: u8) -> Result<u8, &'static str> {
    if n < 10 {
        Ok(n as u8)
    } else {
        Err("digit too large")
    }
}
