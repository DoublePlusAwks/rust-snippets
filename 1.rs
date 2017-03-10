// If we list all the natural numbers below 10 that are multiples of
// 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.

fn euler1(x : &i32) -> i32  {
  (1..*x).filter(|y| y % 3 == 0 || y % 5 == 0).sum()
}

fn main() {
  println!("{}", euler1(&1000));
}