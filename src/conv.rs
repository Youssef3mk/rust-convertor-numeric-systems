use crate::util::*;
pub fn at(haystack: &String, index: usize) -> char {
    for (i, item) in haystack.as_bytes().iter().enumerate() {
        if i == index {
            return *item as char;
        }
    }
    '!'
}
#[derive(Copy, Clone)]
pub enum ENsys {
    BIN = 2,
    HEX = 16,
    DEC = 10,
    OCT = 8,
}
impl ENsys {
    fn value(self) -> u8 {
        self as u8
    }
}
pub fn to_dec(number: &String, sys: ENsys) -> u64 {
    let mut res: u64 = 0;
    for (p, i) in (0..number.len()).rev().enumerate() {
        res = res
            + ((at(number, i).to_digit(sys.value() as u32)).unwrap() as u64)
                * ((sys.value() as u32).pow(p as u32) as u64);
    }
    res
}
pub fn to_bin(number: &String, sys: ENsys) -> u64 {
    let mut res = String::from("");
    let mut dec = match sys {
        ENsys::HEX => to_dec(number, ENsys::HEX),
        ENsys::OCT => to_dec(number, ENsys::OCT),
        ENsys::DEC => number.parse::<u64>().unwrap() as u64,
        ENsys::BIN => 0,
    };
    while dec != 0 {
        res.push(char::from(b'0' + (dec % 2) as u8));

        dec = dec / 2;
    }
    res = res.chars().rev().collect();
    res.parse::<u64>().unwrap()
}
pub fn to_oct(number: &String, sys: ENsys) -> u64 {
    let mut res = String::from("");
    let mut dec = match sys {
        ENsys::HEX => to_dec(number, ENsys::HEX),
        ENsys::OCT => 0,
        ENsys::DEC => number.parse::<u64>().unwrap() as u64,
        ENsys::BIN => to_dec(number, ENsys::BIN),
    };
    while dec != 0 {
        res.push(char::from(b'0' + (dec % 8) as u8));

        dec = dec / 8;
    }
    res = res.chars().rev().collect();
    res.parse::<u64>().unwrap()
}
pub fn to_hex(number: &String, sys: ENsys) -> String {
    let mut res = String::from("");
    let mut dec = match sys {
        ENsys::HEX => 0,
        ENsys::OCT => to_dec(number, ENsys::OCT),
        ENsys::DEC => number.parse::<u64>().unwrap() as u64,
        ENsys::BIN => to_dec(number, ENsys::BIN),
    };
    while dec != 0 {
        res.push(match dec % 16 {
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            _ => char::from(b'0' + (dec % 16) as u8),
        });

        dec = dec / 16;
    }
    res = res.chars().rev().collect();
    res
}
