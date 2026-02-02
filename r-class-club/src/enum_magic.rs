enum Return {
    Something(i32),
    Nothing,
}

fn increment(n: i32) -> Return {
    if n < 0 {
        return Return::Nothing;
    }
    Return::Something(n + 1)
}

fn r_main() {
    match increment(0) {
        Return::Something(x) => {
            println!("The start of natural numbers: {}", x);
        }
        Return::Nothing => {
            println!("We're out of numbers!");
        }
    }
}
