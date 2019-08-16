pub fn raindrops(n: u32) -> String {
    let mut r = String::new();

    if n % 3 == 0 {
        r.push_str("Pling");
    }
    if n % 5 == 0 {
        r.push_str("Plang");
    }
    if n % 7 == 0 {
        r.push_str("Plong");
    }
    if r.len() == 0 {
        r = n.to_string()
    }

    r
}
