use std::io;
fn main() {
    println!("Welcome to my maths rust program \nHere select any of the formuals you like \nAnd input its respective variables");
    println!("t = Area of trapezium = height/2*(base1+base2");
    println!("r = Area of rhombus= (1/2)*diagonal1*diagonal2");
    println!("p = Area of parallelogram = base*altitude");
    println!("cu = Area of cube = 6*(length of the side)^(2)");
    println!("cy = Volume of Cylinder = π*(radius)^2*height");
    let  mut formula=String::new();
    println!("\nEnter any of the letters t,r,p,cu,cy to select a formula to execute/run:");
    io::stdin().read_line(&mut formula).expect("failed to read input");

    if formula.trim()=="t"{
        let mut input1=String::new();
        let mut input2 =String::new();
        let mut input3=String::new();

        println!("enter height");
        io::stdin().read_line(&mut input1).expect("failed to read input");
        let h:f64=input1.trim().parse().expect("Failed to read");
        println!("enter base1");
        io::stdin().read_line(&mut input2).expect("failed to read input");
        let b1:f64=input2.trim().parse().expect("Failed to read");
        
        println!("enter base2");
        io::stdin().read_line(&mut input3).expect("failed to read input");
        let b2:f64=input3.trim().parse().expect("Failed to read");

        let ans:f64 = h/2.0*(b1+b2);

        println!("your answer is  {} ",  ans);
        


    }
    
   else if formula.trim() =="r"{
    let mut input1=String::new();
    let mut input2 =String::new();
    
    
    println!("enter diagnol1");
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let d1:f64=input1.trim().parse().expect("Failed to read");
        println!("enter diagonal2");
        io::stdin().read_line(&mut input2).expect("failed to read input");
        let d2:f64=input2.trim().parse().expect("Failed to read");
       let ans:f64 = (1.0/2.0)*d1*d2;
       println!("your answer is  {} ",  ans);


   }

   else if formula.trim() =="p"{
    let mut input1=String::new();
    let mut input2 =String::new();
    println!("enter base");
    io::stdin().read_line(&mut input1).expect("failed to read input");
        let b:f64=input1.trim().parse().expect("Failed to read");
    println!("enter altitude");
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let a:f64=input2.trim().parse().expect("Failed to read");
   let ans:f64= b*a;
   println!("your answer is  {} ",  ans);

   }
   else if formula.trim() =="cu"{
    let mut input1=String::new();
    println!("enter length of side");
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let los:f64=input1.trim().parse().expect("Failed to read");
 let ans:f64=6.0*(los).powf(2.0);
 println!("your answer is  {} ",  ans);
   }

   else if formula.trim() =="cy"{
    let mut input1=String::new();
    let mut input2 =String::new();
    println!("enter radius");
    io::stdin().read_line(&mut input1).expect("failed to read input");
        let r:f64=input1.trim().parse().expect("Failed to read");
    println!("enter height");
    io::stdin().read_line(&mut input2).expect("failed to read input");
        let h:f64=input2.trim().parse().expect("Failed to read");
 let ans:f64=3.14*(r).powf(2.0)*h;
 println!("your answer is  {} ",  ans);

   }
   
   

}