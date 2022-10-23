pub fn pow_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_mod() {
        assert_eq!(22285208, pow_mod(20151125, 18331560, 33554393));
    }
}
