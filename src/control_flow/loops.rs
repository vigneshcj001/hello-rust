pub fn run() {
    // while loop
    let mut count = 0;
    while count < 5 {
        println!("while loop count = {}", count);
        count += 1;
    }

    // loop (infinite until break)
    let mut num = 0;
    loop {
        num += 1;
        if num == 3 {
            println!("loop hit 3, skipping with continue");
            continue;
        }
        if num > 5 {
            println!("loop breaking at {}", num);
            break;
        }
        println!("loop num = {}", num);
    }

    // for loop with range
    for i in 1..=5 {
        println!("for loop i = {}", i);
    }

    // for loop over array
    let arr = [10, 20, 30, 40];
    for val in arr {
        println!("array element = {}", val);
    }
}
