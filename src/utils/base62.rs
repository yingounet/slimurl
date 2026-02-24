const BASE62_CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    
    let mut chars = Vec::with_capacity(11);
    while n > 0 {
        chars.push(BASE62_CHARS[(n % 62) as usize]);
        n /= 62;
    }
    chars.reverse();
    String::from_utf8(chars).unwrap()
}

pub fn decode(s: &str) -> Option<u64> {
    let mut result: u64 = 0;
    for c in s.bytes() {
        let digit = match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'z' => c - b'a' + 10,
            b'A'..=b'Z' => c - b'A' + 36,
            _ => return None,
        };
        result = result.checked_mul(62)?.checked_add(digit as u64)?;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let test_cases = vec![0, 1, 61, 62, 123, 9999, 12345678, u64::MAX];
        for n in test_cases {
            let encoded = encode(n);
            let decoded = decode(&encoded);
            assert_eq!(decoded, Some(n), "Failed for {}", n);
        }
    }
}
