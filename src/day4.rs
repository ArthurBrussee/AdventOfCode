use std::{collections::HashMap, fs};

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Passport {
    fn parse(val: &str) -> Option<Passport> {
        let kv: HashMap<&str, &str> = val
            .split_whitespace()
            .filter(|&f| f != "")
            .map(|kv| {
                let mut parts = kv.split(':');
                (parts.next().unwrap(), parts.next().unwrap())
            })
            .collect();

        if kv.len() == 7 || kv.len() == 8 {
            Some(Passport {
                byr: kv.get("byr")?.to_string(),
                iyr: kv.get("iyr")?.to_string(),
                eyr: kv.get("eyr")?.to_string(),
                hgt: kv.get("hgt")?.to_string(),
                hcl: kv.get("hcl")?.to_string(),
                ecl: kv.get("ecl")?.to_string(),
                pid: kv.get("pid")?.to_string(),
            })
        } else {
            None
        }
    }

    fn is_valid_parsed(&self) -> bool {
        matches!(self.byr.parse::<u32>(), Ok(1920..=2002))
            && matches!(self.iyr.parse::<u32>(), Ok(2010..=2020))
            && matches!(self.eyr.parse::<u32>(), Ok(2020..=2030))
            && {
                if let Some(cm) = self.hgt.strip_suffix("cm") {
                    matches!(cm.parse(), Ok(150..=193))
                } else if let Some(inches) = self.hgt.strip_suffix("in") {
                    matches!(inches.parse(), Ok(59..=76))
                } else {
                    false
                }
            }
            && {
                matches!(
                    self.ecl.as_str(),
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                )
            }
            && {
                if let Some(hcl) = self.hcl.strip_prefix('#') {
                    hcl.len() == 6 && hcl.chars().all(|c| c.is_ascii_hexdigit())
                } else {
                    false
                }
            }
            && { self.pid.len() == 9 && self.pid.parse::<u64>().is_ok() }
    }
}

pub fn calc() -> (usize, usize) {
    let passports = fs::read_to_string("./inputs/day4.txt")
        .expect("Can't find input file.")
        .replace("\r\n", "\n")
        .split("\n\n")
        .filter_map(Passport::parse)
        .collect::<Vec<_>>();
    (
        passports.len(),
        passports
            .iter()
            .filter(|p| Passport::is_valid_parsed(p))
            .count(),
    )
}
