use std::fs;

const VALID_CHARS: &str = "0123456789abcdef";

#[derive(Default, Debug)]
struct Passport {
    birthday_year: Option<i32>,
    issue_year: Option<i32>,
    expiration_date: Option<i32>,
    height: Option<i32>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn from_map(map: &Vec<String>) -> Self {
        let mut ret = Self::default();
        for pair in map {
            let (key, value) = pair.split_once(":").unwrap();
            match key {
                "byr" => {
                    ret.birthday_year = Some(value.parse().unwrap());
                }
                "iyr" => {
                    ret.issue_year = Some(value.parse().unwrap());
                }
                "eyr" => {
                    ret.expiration_date = Some(value.parse().unwrap());
                }
                "hgt" => {
                    if let Some(num) = value.strip_suffix("cm") {
                        ret.height = Some(num.parse().unwrap());
                    } else if let Some(num) = value.strip_suffix("in") {
                        ret.height = Some((num.parse::<f32>().unwrap() * 2.5399986284) as i32);
                    } else {
                        ret.height = Some(value.parse().unwrap());
                    }
                }
                "hcl" => {
                    ret.hair_color = Some(value.to_string());
                }
                "ecl" => {
                    ret.eye_color = Some(value.to_string());
                }
                "pid" => {
                    ret.passport_id = Some(value.to_string());
                }
                "cid" => {
                    ret.country_id = Some(value.to_string());
                }
                _ => panic!(),
            }
        }
        ret
    }

    fn check_presence(&self) -> bool {
        self.birthday_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_date.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn check_value(&self) -> bool {
        let mut valid = true;
        if let Some(byr) = self.birthday_year {
            valid &= byr >= 1920 && byr <= 2002;
        }
        if let Some(iyr) = self.issue_year {
            valid &= iyr >= 2010 && iyr <= 2020;
        }
        if let Some(eyr) = self.expiration_date {
            valid &= eyr >= 2020 && eyr <= 2030;
        }
        if let Some(height) = self.height {
            valid &= height >= 150 && height <= 193;
        }
        if let Some(hair) = &self.hair_color {
            valid &= hair.len() == 7;
            valid &= hair.chars().next().unwrap() == '#';
            valid &= hair
                .chars()
                .skip(1)
                .find(|c| !VALID_CHARS.contains(*c))
                .is_none();
        }
        if let Some(ecl) = &self.eye_color {
            valid &= ecl == "amb"
                || ecl == "blu"
                || ecl == "brn"
                || ecl == "gry"
                || ecl == "grn"
                || ecl == "hzl"
                || ecl == "oth";
        }
        if let Some(pid) = &self.passport_id {
            valid &= pid.chars().find(|c| !"0123456789".contains(*c)).is_none();
            valid &= pid.chars().rev().skip(9).find(|c| *c != '0').is_none();
        }
        valid
    }
}

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();

    let mut valid_presence = 0;
    let mut valid_values = 0;

    for pass in raw.split("\n\n") {
        let map = pass
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let passport = Passport::from_map(&map);
        if passport.check_presence() {
            valid_presence += 1;
            if passport.check_value() {
                valid_values += 1;
            }
        }
    }
    println!("Passports with all field present: {}", valid_presence);
    println!("Passports with valid values: {}", valid_values);
}
