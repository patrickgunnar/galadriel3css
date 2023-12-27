// some constants for alphanumeric characters, len the alphabetic characters and the seed
const ALPHANUMERIC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const ALPHANUMERIC_LEN: usize = 62;
const SEED: u64 = 5381;

// Cipher is responsible for hashing the receives string
fn cipher(seed: u64, str: &str) -> u64 {
  let mut i = str.len();
  let mut hash = seed;

  while i > 0 {
    i -= 1;
    hash = (hash * 33) ^ str.as_bytes()[i] as u64;
  }

  (hash.wrapping_mul(33)) ^ str.len() as u64
}

// Alphanumeric is responsible for collecting
// a char on the alphanumeric constant
fn alphanumeric(n: usize) -> char {
    ALPHANUMERIC.chars().nth(n).unwrap()
}

// Classinator is responsible to generate a name identifier
// from a received string, than returns the generated name identifier
pub fn classinator(str: &str) -> String {
    let mut name = String::new();
    let code = cipher(SEED, str);
    let mut x = code;

    while x > ALPHANUMERIC_LEN as u64 {
        let remainder = (x % ALPHANUMERIC_LEN as u64) as usize;

        name = alphanumeric(remainder).to_string() + &name;
        x /= ALPHANUMERIC_LEN as u64;
    }

    alphanumeric((x % ALPHANUMERIC_LEN as u64) as usize).to_string() + &name
}
