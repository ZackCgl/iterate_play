fn main() {
    //scaler type: int, bool
    //i8, i16, i32, i64, i128
    //compound type: array, tuple
    //floating variables f32, f64
  let x: f64 = 2.20;
  println!("float = {}", x);
  let is_valueable: bool = false;
  println!("bool is {}", is_valueable);
//single character
  let letter: u8 = 8;
println!("this is a letter {}", letter);
//tuple
let mut tup: (i32, bool, char) = (1, true, 's');
tup = (2, false, 't');
println!("this is a {}", tup.0);

//array
let mut arr : [i32; 6]= [1, 2, 3, 4, 5, 6];
println!("array = {}", arr[3]);

}


