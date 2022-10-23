use std::str;

use compact_str::CompactString;

// inspired by nom
pub type ParseResult<I, O> = Option<(I, O)>;

/// Parse a positive integer.
///
/// * `input`: the input to parse
/// * `ignore_prefix`: whether to ignore any non-digit character occuring before the first digit
pub fn positive(input: &[u8], ignore_prefix: bool) -> ParseResult<&[u8], u64> {
    let mut result = 0;
    let mut count = 0;
    let mut found_digit = false;
    for b in input.iter() {
        match b {
            b'0'..=b'9' => {
                result = result * 10 + (b - b'0') as u64;
                count += 1;
                found_digit = true;
            }
            _ => {
                if ignore_prefix && !found_digit {
                    count += 1;
                    continue;
                }
                break;
            }
        };
    }
    if count > 0 && found_digit {
        return Some((&input[count..], result));
    }
    None
}

/// Parse a signed integer.
///
/// * `input`: the input to parse
/// * `ignore_prefix`: whether to ignore any non-digit character occuring before the first digit
pub fn integer(input: &[u8], ignore_prefix: bool) -> ParseResult<&[u8], i64> {
    let mut start = 0;
    if ignore_prefix {
        for (i, &b) in input.iter().enumerate() {
            if (b'0'..=b'9').contains(&b) {
                // found digit, check if previous character was a minus
                start = i;
                break;
            }
        }
    }
    positive(&input[start..], false).map(|res| {
        (
            res.0,
            if start > 0 && input[start - 1] == b'-' {
                -(res.1 as i64)
            } else {
                res.1 as i64
            },
        )
    })
}

/// Parse a token of the form: a-z, A-Z
///
/// * `input`: the input to parse
pub fn token(input: &[u8]) -> ParseResult<&[u8], CompactString> {
    let mut count: usize = 0;
    for &b in input.iter() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' => {
                count += 1;
            }
            _ => {
                break;
            }
        }
    }
    if count > 0 {
        let token = unsafe { str::from_utf8_unchecked(&input[0..count]) };
        Some((&input[count..], CompactString::new(token)))
    } else {
        None
    }
}

/// Parse a sequence of alphabetic characters: 0-9, a-z, A-Z
///
/// * `input`: the input to parse
pub fn alphanumeric(input: &[u8]) -> ParseResult<&[u8], CompactString> {
    let mut count: usize = 0;
    for &b in input.iter() {
        match b {
            b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => {
                count += 1;
            }
            _ => {
                break;
            }
        }
    }
    if count > 0 {
        let token = unsafe { str::from_utf8_unchecked(&input[0..count]) };
        Some((&input[count..], CompactString::new(token)))
    } else {
        None
    }
}

/// Go to the start of the next line.
/// May return an empty slice.
///
/// * `input`: the input to parse
pub fn seek_next_line(input: &[u8]) -> &[u8] {
    seek(input, b'\n')
}

/// Go one past the specified character.
/// May return an empty slice.
///
/// * `input`: the input to parse
/// * `target`: the character to seek
pub fn seek(input: &[u8], target: u8) -> &[u8] {
    let mut count = 0;
    for &b in input.iter() {
        count += 1;
        if b == target {
            break;
        }
    }
    &input[count..]
}

/// Skip whitespace.
///
/// * `input`: input to parse
pub fn skip_ws(input: &[u8]) -> &[u8] {
    let mut count = 0;
    for &b in input.iter() {
        if b != b' ' {
            break;
        }
        count += 1;
    }
    &input[count..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive() {
        let s = "123 abc";
        let (rest, number) = positive(s.as_bytes(), false).unwrap();
        assert_eq!(123, number);
        assert_eq!(" abc".as_bytes(), rest);

        let s = "foo123 abc";
        let (rest, number) = positive(s.as_bytes(), true).unwrap();
        assert_eq!(123, number);
        assert_eq!(" abc".as_bytes(), rest);
    }

    #[test]
    fn test_integer() {
        let s = "123 abc";
        let (rest, val) = integer(s.as_bytes(), false).unwrap();
        assert_eq!(123, val);
        assert_eq!(" abc".as_bytes(), rest);

        let s = "foo123 abc";
        let (rest, val) = integer(s.as_bytes(), true).unwrap();
        assert_eq!(123, val);
        assert_eq!(" abc".as_bytes(), rest);

        let s = "foo -123 abc";
        let (rest, val) = integer(s.as_bytes(), true).unwrap();
        assert_eq!(-123, val);
        assert_eq!(" abc".as_bytes(), rest);

        let s = "fo - o -123 abc";
        let (rest, val) = integer(s.as_bytes(), true).unwrap();
        assert_eq!(-123, val);
        assert_eq!(" abc".as_bytes(), rest);
    }

    #[test]
    fn test_token() {
        let s = "abc 123";
        let (rest, tk) = token(s.as_bytes()).unwrap();
        assert_eq!("abc", tk);
        assert_eq!(" 123".as_bytes(), rest);

        let s = "FOO bar";
        let (rest, tk) = token(s.as_bytes()).unwrap();
        assert_eq!("FOO", tk);
        assert_eq!(" bar".as_bytes(), rest);
    }

    #[test]
    fn test_alphanumeric() {
        let s = "123abc foo";
        let (rest, an) = alphanumeric(s.as_bytes()).unwrap();
        assert_eq!("123abc", an);
        assert_eq!(" foo".as_bytes(), rest);
    }

    #[test]
    fn test_seek_next_line() {
        let s = "abc\ndef";
        let rest = seek_next_line(s.as_bytes());
        assert_eq!("def".as_bytes(), rest);

        let empty: [u8; 0] = [];

        let s = "abc";
        let rest = seek_next_line(s.as_bytes());
        assert_eq!(empty, rest);

        let s = "abc\n";
        let rest = seek_next_line(s.as_bytes());
        assert_eq!(empty, rest);
    }

    #[test]
    fn test_skip_ws() {
        let s = " abc";
        let rest = skip_ws(s.as_bytes());
        assert_eq!("abc", str::from_utf8(rest).unwrap());

        let s = "      abc";
        let rest = skip_ws(s.as_bytes());
        assert_eq!("abc", str::from_utf8(rest).unwrap());
    }
}
