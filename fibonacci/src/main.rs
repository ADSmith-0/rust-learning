fn main() {
  let mut input = String::new();

  std::io::stdin()
    .read_line(&mut input)
    .expect("Could not read line");

  let input: i32 = match input.trim().parse() {
    Ok(v) => v,
    Err(_) => panic!("Could not convert number!"),
  };

  println!("fibonacci {input}: {}", fibonacci(input));
}

// For loop
// fn fibonacci(n: i32) -> i32 {
//   if n == 0 || n == 1 {
//     n
//   } else {
//     let mut n0 = 0;
//     let mut n1 = 1;
//     let mut n2 = n0 + n1;
//
//     for _ in 1..n {
//       n2 = n0 + n1;
//       n0 = n1;
//       n1 = n2;
//     }
//
//     n2
//   }
// }

// Recursive
fn fibonacci(n: i32) -> i32 {
  if n == 0 || n == 1 {
    n
  } else {
    fibonacci(n - 1) + fibonacci(n - 2)
  }
}
