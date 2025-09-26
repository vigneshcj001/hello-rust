mod integer_float;
mod boolean;
mod string_char;
mod conditional;
mod loops;
mod functions;
mod stack_AND_heap;
fn main() {
    // We can write println! or print!
    println!("=== Welcome to Rust Learning Demo ===\n");

    println!("--- Integer Demo ---");
    integer_float::run(); 

    println!("\n--- Boolean Demo ---");
    boolean::male();

    println!("\n--- String & Char Demo ---");
    string_char::show();

    println!("\n--- Conditional Demo ---");
    conditional::check_number(15);

    println!("\n--- Loops Demo ---");
    loops::run();

    println!("\n--- Functions Demo ---");
    let sum = functions::do_sum(5, 10);
    println!("Sum of 5 and 10 is: {}", sum);

    println!("\n--- Stack and Heap ---");
    stack_AND_heap::stack_heap();
}
