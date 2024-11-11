use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Do you have experience?");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let experience: bool = input1.trim().parse().expect("Failed to parse input");

    println!("Enter your age:");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let age: u32 = input2.trim().parse().expect("Failed to parse input");

    if experience && age >= 40 {
        println!("Your incentive is 1,560,000");
    } else if experience && age >= 30 && age <= 40 {
        println!("Your incentive is 1,480,000");
    } else if experience && age < 28 {
        println!("Your incentive is 1,300,000");
    } else {
        println!("Your incentive is 100,000");
    }
}