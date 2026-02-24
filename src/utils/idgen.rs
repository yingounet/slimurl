use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

use super::encode;

pub fn generate_code() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    
    let random: u32 = rand::thread_rng().gen();
    let id = (now << 22) | ((random as u64) & 0x3FFFFF);
    
    encode(id)
}

pub fn generate_short_code() -> String {
    let mut rng = rand::thread_rng();
    let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut code = Vec::with_capacity(6);
    for _ in 0..6 {
        code.push(chars[rng.gen_range(0..62)]);
    }
    String::from_utf8(code).unwrap()
}
