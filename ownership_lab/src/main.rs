fn shout(s: String) {
    println!("{s}!");
}

fn peek(s: &String) {
    let n: usize = s.len();
    println!("{s} has length {n}");
}

fn excite(s: &mut String) {
    s.push_str("!!!");
}

fn main() {
    let mut name = String::from("ferris");
    peek(&name);
    excite(&mut name);
    println!("{name}");
    shout(name);
}
