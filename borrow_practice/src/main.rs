fn main() {
    let s = String::from("hello");
    let r = &s;        // borrow box #1
    let s = 5;         // shadow: s now names a NEW box holding a number
    println!("{r} {s}");
}