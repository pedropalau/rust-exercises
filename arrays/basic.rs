fn main() {
  let array: [i32; 5] = [1, 2, 3, 4, 5];

  let first = array[1];
  let last = array[4];

  println!("The value of the first element is: {}", first);
  println!("The value of the last element is: {}", last);
  println!("The values on the array are: {:?}", array);
}
