pub fn show() {
    // String slice (&str)
    let greeting: &str = "Hello Rust!";
    println!("Greeting (string slice): {}", greeting);

    // Growable String
    let mut message = String::from("Learning Rust");
    message.push_str(" step by step");
    println!("Message (String): {}", message);

    // Characters
    let letter: char = 'R';
    let emoji: char = '🙂';
    println!("Letter = {}, Emoji = {}", letter, emoji);

    // Concatenation
    let combined = format!("{} {}", greeting, message);
    println!("Combined text: {}", combined);
}
