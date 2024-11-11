use std::io;
fn main() {
    
    //program to calculate the area of three triangles
let mut input1 = String::new();
let mut input2 = String::new();
let mut input3 = String::new();

println!("Enter first edge of triangle:");
io::stdin().read_line(&mut input1 ). expect("Not a Valid String");
let a:f32 = input1.trim().parse().expect("Not a Valid Number");

println!("Enter second edge of triangle:");
io::stdin().read_line(&mut input2 ). expect("Not a Valid String");
let b:f32 = input2.trim().parse().expect("Not a Valid Number");

println!("Enter third edge of triangle:");
io::stdin().read_line(&mut input3 ). expect("Not a Valid String");
let c:f32 = input3.trim().parse().expect("Not a Valid Number");

let s: f32 = (a+b+c)/ 2.0;
let mut area:f32 = s * (s-a) * (s-b ) * (s-c);
area=area.sqrt();

println!("Area of a triangle:{}" ,area );














}
