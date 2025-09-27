pub fn check_number(num: i32) {
    println!(">>> Conditional Demo <<<");
    println!("Number received: {}", num);

    // if-else ladder
    if num > 0 {
        println!("{} is positive", num);
    } else if num < 0 {
        println!("{} is negative", num);
    } else {
        println!("Number is zero");
    }

    // using if as an expression
    let result = if num % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", num, result);

    // match expression (like switch in other languages)
    match num {
        1 => println!("One"),
        2 | 3 => println!("Two or Three"),
        4..=10 => println!("Between 4 and 10"),
        _ => println!("Something else"),
    }
}
