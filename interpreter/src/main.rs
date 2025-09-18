fn eval(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str().to_lowercase().as_str(),
    }
}

fn main() {
    println!("{}", eval(String::from("hello world!!!!")));
}
