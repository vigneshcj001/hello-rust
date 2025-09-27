pub fn male() {
    println!(">>> Boolean Demo <<<");

    let is_male: bool = true;
    let is_above_18: bool = true;

    println!("is_male = {}, is_above_18 = {}", is_male, is_above_18);

    if is_male {
        println!("You are a male");
    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        println!("You are a legal male");
    } else if is_male && !is_above_18 {
        println!("You are a minor male");
    } else {
        println!("You are not male");
    }

    // More boolean examples
    let can_vote = is_above_18;
    println!("Can vote? {}", can_vote);
    println!("Not male = {}", !is_male);
}
