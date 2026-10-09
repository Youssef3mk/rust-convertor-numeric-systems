pub fn at(haystack: &String, index: usize) -> char {
    for (i, item) in haystack.as_bytes().iter().enumerate() {
        if i == index {
            return *item as char;
        }
    }
    '!'
}
