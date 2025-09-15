fn eval(s: String) {
    // Reverse Polish Notation
    let list = s.split("");
    let mut stack: Vec<&str> = Vec::new();
    let mut res: &str = "";
    for i in list {
        if i.chars().nth(0).unwrap().is_digit(10) {
            stack.push(i);
        } else if i == "+" {
            let rhs: i32 = stack.pop().unwrap().parse().unwrap();
            let lhs: i32 = stack.pop().unwrap().parse().unwrap();
            let res = (rhs + lhs).to_string().as_str();
            stack.append(&mut vec![&res]);
        }
    }
}

fn main() {
    eval(String::new());
}
