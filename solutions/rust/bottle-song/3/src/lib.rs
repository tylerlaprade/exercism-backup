const NAMES: [&str; 11] = [
    "no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

fn format_name(num_bottles: usize) -> String {
    format!(
        "{} green bottle{}",
        NAMES[num_bottles],
        if num_bottles == 1 { "" } else { "s" }
    )
}

#[must_use]
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let start_bottles = 1 + start_bottles as usize;
    (start_bottles - take_down as usize..start_bottles)
        .rev()
        .map(|num_bottles| {
            format!(
                "{} hanging on the wall,\n\
                {0} hanging on the wall,\n\
                And if one green bottle should accidentally fall,\n\
                There'll be {} hanging on the wall.",
                format_name(num_bottles),
                format_name(num_bottles - 1).to_lowercase()
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}
