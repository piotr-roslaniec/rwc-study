// TODO: Takes a really long time to run and doesn't produce the correct answer.

fn main() {
    let original_input = b"original_input";
    println!("Original input: {:x?}", original_input);

    let original_digest: [u8; 16] = md5::compute(original_input).try_into().unwrap();
    println!("Original digest: {:x?}", original_digest);

    let bytes_for_collision = 4;
    println!("Bytes for collision: {}", bytes_for_collision);

    let mut attempts: i128 = 0;
    loop {
        let new_input = attempts.to_ne_bytes();
        let new_digest: [u8; 16] = md5::compute(new_input).try_into().unwrap();

        if new_digest[..bytes_for_collision] == original_digest[..bytes_for_collision] {
            println!("Collision found: {:x?}", new_digest);
            println!("Hash input: {:x?}", new_input);
            println!("Attempts: {}", attempts);
            break;
        }
        attempts += 1;
    }
}
