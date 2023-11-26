fn main() {
    // let reference_to_nothing = dangle();
    first_word(&String::from("hello world"));
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
