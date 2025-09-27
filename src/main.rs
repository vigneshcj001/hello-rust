mod basics {
    pub mod integer_float;
    pub mod boolean;
    pub mod string_char;
    pub mod conditional;
}

mod control_flow {
    pub mod loops;
    pub mod functions;
}

mod memory {
    pub mod stack_and_heap;
}

mod ownership {
    pub mod ownership_demo1;
    pub mod ownership_demo2;
}

fn main() {
    println!("=== Welcome to Rust Learning Demo ===\n");

    basics::integer_float::run();
    basics::boolean::male();
    basics::string_char::show();
    basics::conditional::check_number(15);

    control_flow::loops::run();
    let sum = control_flow::functions::do_sum(5, 10);
    println!("Sum of 5 and 10 is: {}", sum);

    memory::stack_and_heap::stack_heap();

    ownership::ownership_demo1::run();
    ownership::ownership_demo2::run();
}
