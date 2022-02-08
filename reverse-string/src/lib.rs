use unicode_segmentation::UnicodeSegmentation;

pub fn reverse0(input: &str) -> String {
    input.graphemes(true).rev().flat_map(|c| c.chars()).collect()
}

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
