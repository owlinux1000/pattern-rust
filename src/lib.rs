const UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";

pub fn create(length: usize) -> String {
    let mut payload: Vec<String> = Vec::new();
    for c in UPPER_CASE.chars() {
        for s in LOWER_CASE.chars() {
            for n in NUMBERS.chars() {
                if payload.len() < length {
                    payload.push(c.to_string());
                }
                if payload.len() < length {
                    payload.push(s.to_string());
                }
                if payload.len() < length {
                    payload.push(n.to_string());    
                }
            }
        }
    }
    payload.join("")
}

#[test]
fn test_create() {
    let expected = "Aa0Aa1Aa2Aa3Aa4Aa5Aa6Aa7Aa8Aa9Ab0Ab1Ab2Ab3Ab4Ab5Ab";
    let actual = create(50);
    assert_eq!(expected, actual);
}


pub fn offset(value: &str, flag: bool) -> Option<usize> {
    let payload = create(20280);
    if let Some(i) = payload.find(value) {
        return Some(i);
    } else {
        if let Ok(h) = u64::from_str_radix(value, 16) {
            if let Some(i) = payload.find(&hex2ascii(h, flag)) {
                return Some(i);
            }
        }
        return None;
    }
}

#[test]
fn test_offset() {
    let expected = 26;
    let actual = offset("8Aa9", false).unwrap();
    assert_eq!(expected, actual);
    let expected = 0;
    let actual = offset("Aa0", false).unwrap();
    assert_eq!(expected, actual);
    let expected = 20277;
    let actual = offset("Zz9", false).unwrap();
    assert_eq!(expected, actual);
}

fn hex2ascii(hex: u64, bigendian_flag: bool) -> String {
    let mut s: Vec<String> = Vec::new();
    let mut h = hex;
    while h != 0 {
        s.push(
            format!("{}", (h & 0xff) as u8 as char)
        );
        h >>= 8;
    }
    if bigendian_flag {
        s.reverse();
    }
    s.join("")
}

#[test]
fn test_hex2ascii() {
    let expected = String::from("AAAA");
    let actual = hex2ascii(0x41414141, false);
    assert_eq!(expected, actual);

    let expected = String::from("ABCD");
    let actual = hex2ascii(0x44434241, false);
    assert_eq!(expected, actual);

    // Testing with big endian
    let expected = String::from("ABCD");
    let actual = hex2ascii(0x41424344, true);
    assert_eq!(expected, actual);
}
