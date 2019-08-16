pub fn verse(n: i32) -> String {
    let bottles = |x| {
        format!(
            "{} bottle{} of beer",
            if x == 0 {
                String::from("no more")
            } else {
                format!("{}", x)
            },
            if x > 1 || x == 0 { "s" } else { "" }
        )
    };
    let wall = |x| format!("{} on the wall", bottles(x));
    if n == 0 {
        format!(
            "No more bottles of beer on the wall, {}.\nGo to the store and buy some more, {}.\n",
            bottles(n),
            wall(99)
        )
    } else {
        format!(
            "{}, {}.\nTake {} down and pass it around, {}.\n",
            wall(n),
            bottles(n),
            if n > 1 { "one" } else { "it" },
            wall(n - 1)
        )
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start)
        .map(verse)
        .rev()
        .collect::<Vec<String>>()
        .join("\n")
}
