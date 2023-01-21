use std::{
    fs,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Snafu(i64);

impl Deref for Snafu {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Snafu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Snafu {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut val: i64 = 0;
        for (i, c) in s.chars().enumerate() {
            let exp = 5_i64.pow((s.len() - i - 1) as u32);
            val += Snafu::digit_to_value(c) * exp;
        }
        Ok(Self(val))
    }
}

// n base 5 = d_0*(5^0) + d_1*(5^1) + ... + d_k*(5^k)
//   d_0, d_1, ..., d_k in 0..5
// m snafu = (d_0 - 2)*(5^0) + (d_1 - 2)*(5^1) + ... + (d_k - 2)*(5^k)
//         = -2*5^0 + d_0*5^0 - 2*5^1 + d_1*5^1 + ... - 2*5^k + d_k*5^k
//         = -2*(5^0 + 5^1 ... + 5^k) + m base 5

impl ToString for Snafu {
    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut val = **self;
        let mut digits = vec![0; 64];
        let mut exp = 0;
        while val > 0 {
            let rem = val % 5;
            val /= 5;
            if rem <= 2 {
                digits[exp] += rem;
            } else {
                digits[exp + 1] += 1;
                if rem == 3 {
                    digits[exp] -= 2;
                }
                if rem == 4 {
                    digits[exp] -= 1;
                }
            }
            exp += 1;
        }
        // only one pass may be enough
        for i in 0..digits.len() {
            if digits[i] >= 3 {
                digits[i + 1] += 1;
                if digits[i] == 3 {
                    digits[i] = -2;
                }
                if digits[i] == 4 {
                    digits[i] = -1;
                }
            }
        }
        let last = digits
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, &d)| d != 0)
            .unwrap()
            .0;
        for i in 0..=last {
            s += &Snafu::value_to_digit(digits[i]).to_string();
        }
        s.chars().rev().collect()
    }
}

impl Snafu {
    fn digit_to_value(d: char) -> i64 {
        match d {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => unreachable!(),
        }
    }
    fn value_to_digit(d: i64) -> char {
        match d {
            0 => '0',
            1 => '1',
            2 => '2',
            -1 => '-',
            -2 => '=',
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sum: i64 = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| *l.parse::<Snafu>().unwrap())
        .sum();
    println!("Sum of fuel in decimal: {}", sum);
    let snafu = Snafu(sum);
    assert_eq!(Snafu::from_str(&snafu.to_string()).unwrap(), snafu);
    println!("Sum of fuel in snafu: {}", Snafu(sum).to_string());
}

#[cfg(test)]
mod test {
    use crate::*;

    const DECIMAL_TO_SNAFU: [(&str, &str); 15] = [
        ("1", "1"),
        ("2", "2"),
        ("3", "1="),
        ("4", "1-"),
        ("5", "10"),
        ("6", "11"),
        ("7", "12"),
        ("8", "2="),
        ("9", "2-"),
        ("10", "20"),
        ("15", "1=0"),
        ("20", "1-0"),
        ("2022", "1=11-2"),
        ("12345", "1-0---0"),
        ("314159265", "1121-1110-1=0"),
    ];

    #[test]
    fn decimal_to_snafu() {
        for (dec, sn) in DECIMAL_TO_SNAFU {
            dbg!(dec, sn);
            assert_eq!(Snafu(dec.parse::<i64>().unwrap()).to_string(), sn);
        }
    }

    const SNAFU_TO_DECIMAL: [(&str, &str); 13] = [
        ("1=-0-2", "1747"),
        ("12111", "906"),
        ("2=0=", "198"),
        ("21", "11"),
        ("2=01", "201"),
        ("111", "31"),
        ("20012", "1257"),
        ("112", "32"),
        ("1=-1=", "353"),
        ("1-12", "107"),
        ("12", "7"),
        ("1=", "3"),
        ("122", "37"),
    ];

    #[test]
    fn snafu_to_decimal() {
        for (sn, dec) in SNAFU_TO_DECIMAL {
            dbg!(dec, sn);
            assert_eq!(dec.parse::<i64>().unwrap(), *sn.parse::<Snafu>().unwrap());
        }
    }
}
