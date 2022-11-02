use std::fs;

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

    fn check(&self) -> bool {
        self.birthday_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_date.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
}

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();

    let mut valid = 0;

    for pass in raw.split("\n\n") {
        let map = pass
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let passport = Passport::from_map(&map);
        if passport.check() {
            valid += 1;
        }
    }
    println!("{} valid passports", valid);
}
