fn main() {
   //initialization of tuple with data type
   let datatype_tuple:(&str, f32, u32) = ("Rust", 3.14, 100);
   ("Tuple contents = {:?} " , datatype_tuple);
   //initialization of tuple without data type
   let no_datatype_tuple = ("Rust" , "Fun" , 100);
   println!("Tuple contents = {:?} " , no_datatype_tuple);
   //accesing tuple element at index 0
   println!("Value At index 0 ={}" , datatype_tuple.0);
   
   //accesing tuple element at index 1
   println!("Value At index 1 ={}" , datatype_tuple.1);
      //accesing tuple element at index 2
      println!("Value At index 2 ={}" , datatype_tuple.2);
}