pub fn run() {
    let my_string = String::from("hello");
    takes_ownership(my_string);

    // Uncommenting this will cause error because my_string was moved
    // println!("{}", my_string);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
