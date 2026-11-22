fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}

fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut last_word_start_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            last_word_start_index = i + 1;
        }
    }

    &s[last_word_start_index..]
}

fn count_spaces(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut spaces_count = 0;

    for (_i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            spaces_count += 1;
        }
    }

    spaces_count
}

fn main() {
    let text = String::from("rust is pretty fun");
    println!("The first word is: {}", first_word(&text));
    println!("The last word is: {}", last_word(&text));
    println!("The number of spaces is: {}", count_spaces(&text));

    println!("The first word is: {}", first_word("Hello World"));
    println!("The first word is: {}", first_word("Rust"));
}
