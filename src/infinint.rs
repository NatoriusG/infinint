use std::{fmt, ops};

pub struct Infinint {
    negative: bool,
    digits_vec: Vec<u8>,
}

#[allow(dead_code)]
impl Infinint {
    pub fn new() -> Infinint {
        Infinint {
            negative: false,
            digits_vec: vec![0],
        }
    }

    fn digits(&self) -> Vec<u8> {
        let mut digits_vector = Vec::with_capacity(self.digits_vec.len() * 2);

        for byte in &self.digits_vec {
            let digit_pair = decimal_digits(*byte).unwrap();
            digits_vector.push(digit_pair.0);
            digits_vector.push(digit_pair.1);
        }
        match digits_vector.last() {
            Some(d) if *d == 0 => digits_vector.pop(),
            _ => None,
        };

        digits_vector
    }

    fn digits_vec_from_int(n: u128) -> Vec<u8> {
        let mut n = n;

        let bytes_needed = match n {
            0 => 1,
            _ => (((n as f64).abs().log10()) as usize / 2) + 1,
        };
        let next_exp = (bytes_needed as f64).log2().ceil();
        let next_pow_of_two = 2.0_f64.powi(next_exp as i32);
        let mut digits_vec: Vec<u8> = Vec::with_capacity(next_pow_of_two as usize);

        if n > 0 {
            while n > 0 {
                let mut d: u8;

                let n_mod = (n % 10) as u8;
                d = n_mod << 4;
                n /= 10;

                let n_mod = (n % 10) as u8;
                d = n_mod | d;
                n /= 10;

                digits_vec.push(d);
            }
        } else {
            digits_vec.push(0);
        }

        digits_vec
    }
}

impl fmt::Debug for Infinint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_out = String::new();
        debug_out.push_str(format!("\nnegative: {}\n", self.negative).as_str());
        debug_out.push_str(format!("digits: [\n").as_str());
        for d in &self.digits_vec {
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
        debug_out.push_str("]");
        write!(f, "{}", debug_out)
    }
}

impl fmt::Display for Infinint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut raw_digits = self.digits();

        let num_digits = raw_digits.len();
        let num_chars = num_digits
            + if !f.alternate() {
                (num_digits - 1) / 3
            } else {
                0
            };

        let mut number = String::with_capacity(num_chars);

        for i in 0..num_chars {
            number.push(if !f.alternate() && (num_chars - i) % 4 == 0 {
                ','
            } else {
                std::char::from_digit(raw_digits.pop().unwrap().into(), 10).unwrap()
            });
        }

        f.pad_integral(!self.negative, "", &number)
    }
}

impl From<u128> for Infinint {
    fn from(n: u128) -> Infinint {
        let digits_vec = Infinint::digits_vec_from_int(n);

        Infinint {
            negative: false,
            digits_vec,
        }
    }
}

impl From<i128> for Infinint {
    fn from(n: i128) -> Infinint {
        let negative = n < 0;
        let digits_vec = Infinint::digits_vec_from_int(n.abs() as u128);

        Infinint {
            negative,
            digits_vec,
        }
    }
}

impl From<usize> for Infinint {
    fn from(n: usize) -> Infinint {
        // since usize < u128, conversion is safe
        Infinint::from(n as u128)
    }
}

impl From<isize> for Infinint {
    fn from(n: isize) -> Infinint {
        // since isize < i128, conversion is safe
        Infinint::from(n as i128)
    }
}

impl From<u64> for Infinint {
    fn from(n: u64) -> Infinint {
        Infinint::from(u128::from(n))
    }
}

impl From<i64> for Infinint {
    fn from(n: i64) -> Infinint {
        Infinint::from(i128::from(n))
    }
}

impl From<u32> for Infinint {
    fn from(n: u32) -> Infinint {
        Infinint::from(u128::from(n))
    }
}

impl From<i32> for Infinint {
    fn from(n: i32) -> Infinint {
        Infinint::from(i128::from(n))
    }
}

impl From<u16> for Infinint {
    fn from(n: u16) -> Infinint {
        Infinint::from(u128::from(n))
    }
}

impl From<i16> for Infinint {
    fn from(n: i16) -> Infinint {
        Infinint::from(i128::from(n))
    }
}

impl From<u8> for Infinint {
    fn from(n: u8) -> Infinint {
        Infinint::from(u128::from(n))
    }
}

impl From<i8> for Infinint {
    fn from(n: i8) -> Infinint {
        Infinint::from(i128::from(n))
    }
}

impl ops::Add<&Infinint> for &Infinint {
    type Output = Infinint;

    fn add(self, other: &Infinint) -> Infinint {
        if self.negative == false && other.negative == true {
            // n + -m = n - m : short circuit return subtraction op
        } else if self.negative == true && other.negative == false {
            // -n + m = m - n : short circuit return subtraction op
        } // otherwise, negative can be determined later

        let mut self_iter = self.digits_vec.iter();
        let mut other_iter = other.digits_vec.iter();
        let mut carry = 0;
        let mut result_digits_vec: Vec<u8> = Vec::with_capacity(
            2 * std::cmp::max(self.digits_vec.capacity(), other.digits_vec.capacity()),
        );

        loop {
            let self_next_digits = *self_iter.next().unwrap_or(&0);
            let other_next_digits = *other_iter.next().unwrap_or(&0);

            if self_next_digits == 0 && other_next_digits == 0 {
                break;
            }

            let self_next_digits = decimal_digits(self_next_digits).unwrap();
            let other_next_digits = decimal_digits(other_next_digits).unwrap();

            let (upper_result_digit, new_carry) =
                decimal_add_with_carry(self_next_digits.0, other_next_digits.0, carry);
            carry = new_carry;

            let (lower_result_digit, new_carry) =
                decimal_add_with_carry(self_next_digits.1, other_next_digits.1, carry);
            carry = new_carry;

            let result_digit = (upper_result_digit << 4) | lower_result_digit;

            if result_digit != 0 {
                result_digits_vec.push(result_digit);
            }
        }

        if carry > 0 {
            result_digits_vec.push(carry);
        }

        if result_digits_vec.len() == 0 {
            result_digits_vec.push(0);
        }

        // since the first lines short-circuit return if the signs of self and other are different,
        // we can assume self and other have the same sign. if that is the case, the sign of the result
        // is the sign of both of the inputs, and since they are the same, we only have to check one.
        Infinint {
            negative: self.negative,
            digits_vec: result_digits_vec,
        }
    }
}

impl ops::Sub<&Infinint> for &Infinint {
    type Output = Infinint;

    fn sub(self, other: &Infinint) -> Infinint {
        // check for nonstandard sub

        let mut self_iter = self.digits_vec.iter();
        let mut other_iter = other.digits_vec.iter();
        let mut carry = 0;
        let mut result_digits_vec: Vec<u8> = Vec::with_capacity(std::cmp::max(
            self.digits_vec.capacity(),
            other.digits_vec.capacity(),
        ));

        loop {
            let self_next_digits = *self_iter.next().unwrap_or(&0);
            let other_next_digits = *other_iter.next().unwrap_or(&0);

            if self_next_digits == 0 && other_next_digits == 0 {
                break;
            }

            let self_next_digits = decimal_digits(self_next_digits).unwrap();
            let other_next_digits = decimal_digits(other_next_digits).unwrap();

            let (upper_result_digit, new_carry) =
                decimal_subtract_with_carry(self_next_digits.0, other_next_digits.0, carry);
            carry = new_carry;

            let (lower_result_digit, new_carry) =
                decimal_subtract_with_carry(self_next_digits.1, other_next_digits.1, carry);
            carry = new_carry;

            let result_digit = (upper_result_digit << 4) | lower_result_digit;

            if result_digit != 0 {
                result_digits_vec.push(result_digit);
            }
        }

        if result_digits_vec.len() == 0 {
            result_digits_vec.push(0);
        }

        Infinint {
            negative: false,
            digits_vec: result_digits_vec,
        }
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
        Ok(n)
    } else {
        Err("digit too large")
    }
}

fn decimal_add_with_carry(n: u8, m: u8, carry: u8) -> (u8, u8) {
    let result = n + m + carry;
    let carry = result / 10;
    let result = result % 10;
    (result, carry)
}

fn decimal_subtract_with_carry(n: u8, m: u8, carry: u8) -> (u8, u8) {
    let (result, carry) = if n >= (m + carry) {
        (n - m - carry, 0)
    } else {
        ((n + 10) - m - carry, 1)
    };
    (result, carry)
}
