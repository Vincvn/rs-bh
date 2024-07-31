pub fn string(len: i32) -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = (0..len)
    .map(|_| rand::Rng::sample(&mut rng, rand::distributions::Alphanumeric) as char)
    .collect();
    random_string
}