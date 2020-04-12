use std::{cmp, fmt, ops};

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

    fn cmp_digits(n_digits_vec: &Vec<u8>, m_digits_vec: &Vec<u8>) -> cmp::Ordering {
        let mut self_iter = n_digits_vec.iter().rev();
        let mut other_iter = m_digits_vec.iter().rev();

        loop {
            let n_next_digits = *self_iter.next().unwrap_or(&0);
            let m_next_digits = *other_iter.next().unwrap_or(&0);

            if n_next_digits == 0 && m_next_digits == 0 {
                return cmp::Ordering::Equal;
            }

            let n_next_digits = decimal_digits(n_next_digits).unwrap();
            let m_next_digits = decimal_digits(m_next_digits).unwrap();

            if n_next_digits.1 < m_next_digits.1 {
                return cmp::Ordering::Less;
            } else if n_next_digits.1 > m_next_digits.1 {
                return cmp::Ordering::Greater;
            }

            if n_next_digits.0 < m_next_digits.0 {
                return cmp::Ordering::Less;
            } else if n_next_digits.0 > m_next_digits.0 {
                return cmp::Ordering::Greater;
            }
        }
    }

    fn infinint_cmp(n: &Infinint, m: &Infinint, negate_n: bool, negate_m: bool) -> cmp::Ordering {
        let n_negative = if negate_n == false {
            n.negative
        } else {
            !n.negative
        };
        let m_negative = if negate_m == false {
            m.negative
        } else {
            !m.negative
        };

        if n_negative == true && m_negative == false {
            cmp::Ordering::Less
        } else if n_negative == false && m_negative == true {
            return cmp::Ordering::Greater;
        } else {
            if n.digits_vec.len() < m.digits_vec.len() {
                cmp::Ordering::Less
            } else if n.digits_vec.len() > m.digits_vec.len() {
                cmp::Ordering::Greater
            } else {
                let digits_ordering = Infinint::cmp_digits(&n.digits_vec, &m.digits_vec);

                if n_negative == true {
                    digits_ordering.reverse()
                } else {
                    digits_ordering
                }
            }
        }
    }

    fn infinint_add(
        n: &Infinint,
        m: &Infinint,
        negate_n: bool,
        negate_m: bool,
        negate_result: bool,
    ) -> Infinint {
        let n_negative = if negate_n == false {
            n.negative
        } else {
            !n.negative
        };
        let m_negative = if negate_m == false {
            m.negative
        } else {
            !m.negative
        };

        if n_negative == false && m_negative == true {
            return Infinint::infinint_subtract(n, m, negate_n, !negate_m, negate_result);
        } else if n_negative == true && m_negative == false {
            return Infinint::infinint_subtract(m, n, negate_m, !negate_n, negate_result);
        } // otherwise, negative can be determined later

        let mut n_iter = n.digits_vec.iter();
        let mut m_iter = m.digits_vec.iter();
        let mut carry = 0;
        let mut result_digits_vec: Vec<u8> =
            Vec::with_capacity(2 * cmp::max(n.digits_vec.capacity(), m.digits_vec.capacity()));

        loop {
            let n_next_digits = *n_iter.next().unwrap_or(&0);
            let m_next_digits = *m_iter.next().unwrap_or(&0);

            if n_next_digits == 0 && m_next_digits == 0 {
                break;
            }

            let n_next_digits = decimal_digits(n_next_digits).unwrap();
            let m_next_digits = decimal_digits(m_next_digits).unwrap();

            let (upper_result_digit, new_carry) =
                decimal_add_with_carry(n_next_digits.0, m_next_digits.0, carry);
            carry = new_carry;

            let (lower_result_digit, new_carry) =
                decimal_add_with_carry(n_next_digits.1, m_next_digits.1, carry);
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

        let result_negative = if negate_result == false {
            n_negative
        } else {
            !n_negative
        };

        Infinint {
            negative: result_negative,
            digits_vec: result_digits_vec,
        }
    }

    fn infinint_subtract(
        n: &Infinint,
        m: &Infinint,
        negate_n: bool,
        negate_m: bool,
        negate_result: bool,
    ) -> Infinint {
        let n_negative = if negate_n == false {
            n.negative
        } else {
            !n.negative
        };
        let m_negative = if negate_m == false {
            m.negative
        } else {
            !m.negative
        };

        if n_negative == false && m_negative == true {
            return Infinint::infinint_add(n, m, negate_n, !negate_m, negate_result);
        } else if n_negative == true && m_negative == false {
            return Infinint::infinint_add(n, m, !negate_n, negate_m, !negate_result);
        } else if n_negative == true && m_negative == true {
            return Infinint::infinint_subtract(m, n, !negate_m, !negate_n, negate_result);
        }

        match Infinint::infinint_cmp(n, m, negate_n, negate_m) {
            cmp::Ordering::Less => {
                return Infinint::infinint_subtract(m, n, negate_m, negate_n, !negate_result);
            }
            cmp::Ordering::Equal => return Infinint::from(0),
            cmp::Ordering::Greater => (),
        }

        let mut n_iter = n.digits_vec.iter();
        let mut m_iter = m.digits_vec.iter();
        let mut carry = 0;
        let mut result_digits_vec: Vec<u8> =
            Vec::with_capacity(cmp::max(n.digits_vec.capacity(), m.digits_vec.capacity()));

        loop {
            let n_next_digits = *n_iter.next().unwrap_or(&0);
            let m_next_digits = *m_iter.next().unwrap_or(&0);

            if n_next_digits == 0 && m_next_digits == 0 {
                break;
            }

            let n_next_digits = decimal_digits(n_next_digits).unwrap();
            let m_next_digits = decimal_digits(m_next_digits).unwrap();

            let (upper_result_digit, new_carry) =
                decimal_subtract_with_carry(n_next_digits.0, m_next_digits.0, carry);
            carry = new_carry;

            let (lower_result_digit, new_carry) =
                decimal_subtract_with_carry(n_next_digits.1, m_next_digits.1, carry);
            carry = new_carry;

            let result_digit = (upper_result_digit << 4) | lower_result_digit;

            if result_digit != 0 {
                result_digits_vec.push(result_digit);
            }
        }

        if result_digits_vec.len() == 0 {
            result_digits_vec.push(0);
        }

        let result_negative = if negate_result == false { false } else { true };

        Infinint {
            negative: result_negative,
            digits_vec: result_digits_vec,
        }
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

impl cmp::Ord for Infinint {
    fn cmp(&self, other: &Infinint) -> cmp::Ordering {
        Infinint::infinint_cmp(self, other, false, false)
    }
}

impl cmp::PartialOrd for Infinint {
    fn partial_cmp(&self, other: &Infinint) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Eq for Infinint {}

impl cmp::PartialEq for Infinint {
    fn eq(&self, other: &Infinint) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

impl ops::Neg for &Infinint {
    type Output = Infinint;

    fn neg(self) -> Infinint {
        let new_negative = !self.negative;
        let mut new_digits_vec = Vec::with_capacity(self.digits_vec.capacity());
        new_digits_vec.resize(self.digits_vec.len(), 0);
        new_digits_vec.copy_from_slice(&self.digits_vec[..]);

        Infinint {
            negative: new_negative,
            digits_vec: new_digits_vec,
        }
    }
}

impl ops::Add<&Infinint> for &Infinint {
    type Output = Infinint;

    fn add(self, other: &Infinint) -> Infinint {
        Infinint::infinint_add(self, other, false, false, false)
    }
}

impl ops::Sub<&Infinint> for &Infinint {
    type Output = Infinint;

    fn sub(self, other: &Infinint) -> Infinint {
        Infinint::infinint_subtract(self, other, false, false, false)
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
