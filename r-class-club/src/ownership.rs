fn owner() {
    let x: String = String::from("Hello World");
    println!("The x phrase: {}", x);
    let y = x;
    println!("The y phrase: {}", y);
    // println!("The x phrase: {}", x);

    // Immutable Borrow
    let z = &y;
    println!("The z phrase: {}", z);
}
