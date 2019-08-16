pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut s = vec![String::from(""); list.len()];
    for i in 0..list.len() - 1 {
        s[i] = format!("For want of a {} the {} was lost.", list[i], list[i + 1]);
    }
    s[list.len() - 1] = format!("And all for the want of a {}.", list[0]);
    s.join("\n")
}
