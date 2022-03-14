use std::io;

fn main() {
    println!("Hello, world!");
    let mut buffer = String::new();
    println!("Enter a message: ");
    io::stdin().read_line(&mut buffer);
    println!("Buffer is {}", buffer);

    //can also do number : i32 = 
    let number = buffer.trim().parse::<i32>().unwrap();

    println!("number + 1 is {}", number + 1);
}
