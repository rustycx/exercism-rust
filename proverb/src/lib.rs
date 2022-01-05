pub fn build_proverb0(list: &[&str]) -> String {
    let mut sentences: Vec<String> = list
        .windows(2)
        .map(|words| format!("For want of a {} the {} was lost.", words[0], words[1]))
        .collect();
    if list.len() > 0 {
        sentences.push(format!("And all for the want of a {}.", list[0]));
    }
    sentences.join("\n")
}

pub fn build_proverb1(list: &[&str]) -> String {
    if list.is_empty() {
        String::new()
    } else {
        list.windows(2)
            .map(|words| format!("For want of a {} the {} was lost.", words[0], words[1]))
            .chain(std::iter::once(format!("And all for the want of a {}.", list[0])))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub fn build_proverb(list: &[&str]) -> String {
    list.iter()
        .zip(list.iter().skip(1))
        .map(|(a, b)| format!("For want of a {} the {} was lost.", a, b))
        .chain(list.iter().take(1).map(|a| format!("And all for the want of a {}.", a)))
        .collect::<Vec<_>>()
        .join("\n")
}
