pub fn run() {
    println!(">>> Ownership Demo 2: Passing Ownership to a Function <<<");

    let my_string = String::from("hello");
    println!("Before passing to function: {}", my_string);

    takes_ownership(my_string);

    // println!("{}", my_string); // ❌ Compile error: my_string moved into function
}

fn takes_ownership(some_string: String) {
    println!("Inside function, ownership received: {}", some_string);
}
