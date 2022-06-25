use sha256;

fn main() {
    const N_OF_TRAILING_ZEROES: usize = 1;
    let pattern = &[0; N_OF_TRAILING_ZEROES];
    let pattern = String::from_utf8_lossy(pattern).to_string();

    let mut attempts: i128 = 0;
    loop {
        let input = attempts.to_ne_bytes();
        let digest = sha256::digest_bytes(&input);
        if digest.ends_with(&pattern) {
            println!("Digest found: {:x?}", digest.as_bytes());
            println!("Hash input: {:x?}", input);
            println!("Attempts: {}", attempts);
            break;
        }
        attempts += 1;
    }
}
