pub fn run() {
    println!(">>> Ownership Demo 1: Move Semantics <<<");

    let s1 = String::from("hello");
    println!("s1 before move: {}", s1);

    let s2 = s1; // Ownership of the string moves to s2
    // println!("{}", s1); // ❌ Compile error: s1 no longer owns the value

    println!("s2 after move: {}", s2);
}
