pub fn create_id() -> i32 {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32;
    let random = rand::random::<u8>();
    (timestamp << 4) | random as i32
}
