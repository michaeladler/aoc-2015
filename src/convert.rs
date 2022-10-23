pub fn byte_to_hex(b: u8) -> (char, char) {
    let q = b / 16;
    let r = b % 16;
    (dec_to_hex(q), dec_to_hex(r))
}

pub fn dec_to_hex(d: u8) -> char {
    match d {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        15 => 'F',
        _ => panic!("dec_to_hex"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_to_hex() {
        assert_eq!(byte_to_hex(255), ('F', 'F'));
        assert_eq!(byte_to_hex(128), ('8', '0'));
        assert_eq!(byte_to_hex(42), ('2', 'A'));
        assert_eq!(byte_to_hex(0), ('0', '0'));
    }
}
