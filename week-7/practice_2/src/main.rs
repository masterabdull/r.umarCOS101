use std::io;
fn checker(){
let mut input=String::new();
println!("Enter a character:");
io::stdin().read_line(&mut input).expect("Failed to read input");
let ch:i32=input.trim().parse().expect("Invalid input");
if ch>= 0 && ch <=9
{
println!("Character '{}' is a digital",ch);

}
else
{
 println!("Character '{}' is not a digit" , ch)   
}

}

fn main() {
    //calling a function
    println!("Welcome! This progran chrcles whther a character variable contains a digit or not");
  checker()
}