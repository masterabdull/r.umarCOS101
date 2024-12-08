use std::io;
fn main() {
    println!("Welcome to the reqruitment program for Ernst and Young  Global limited");

    let mut input = String::new();
    println!("How many candidates do you want to enter?");
    io::stdin().read_line(&mut input).expect("failed to read input");
    let n: u32 = input.trim().parse().expect("failed to parse input");

    let mut candidates_vector: Vec<(String, f32)> = Vec::new();

    for x in 0..n {
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Enter name");
        io::stdin().read_line(&mut input1).expect("failed to input");
        let name: String = input1.trim().parse().expect("failed to parse input");

        println!("Enter number of years of experience");
        io::stdin().read_line(&mut input2).expect("failed to input");
        let years_of_exp: f32 = input2.trim().parse().expect("invalid input");

        let candidate: (String, f32) = (name, years_of_exp);
        candidates_vector.push(candidate);

        println!("Added candidate {}", x);
    }

    candidates_vector.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let exp_candidate = &candidates_vector[0];

    println!("The most experienced candidate is {} with {} years of experience", exp_candidate.0, exp_candidate.1);

}