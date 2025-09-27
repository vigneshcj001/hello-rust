mod basics{
    pub mod integer_float;
    pub mod boolean;
    pub mod string_char;
    pub mod conditional;
}
mod control_flow{
    pub mod loops;
    pub mod functions;
}
mod memory{
    pub mod stack_and_heap;
}
mod ownership{
    pub mod ownership_demo;
}
fn main() {
    // We can write println! or print!
    println!("=== Welcome to Rust Learning Demo ===\n");

    println!("--- Integer Demo ---");
    basics::integer_float::run();

    println!("\n--- Boolean Demo ---");
    basics::boolean::male();

    println!("\n--- String & Char Demo ---");
    basics::string_char::show();

    println!("\n--- Conditional Demo ---");
    basics::conditional::check_number(15);

    println!("\n--- Loops Demo ---");
    control_flow::loops::run();

    println!("\n--- Functions Demo ---");
    let sum = control_flow::functions::do_sum(5, 10);
    println!("Sum of 5 and 10 is: {}", sum);

    println!("\n--- Stack and Heap ---");
    memory::stack_and_heap::stack_heap();

    println!("\n--- Ownership demo ---");
    ownership::ownership_demo::run();
}
