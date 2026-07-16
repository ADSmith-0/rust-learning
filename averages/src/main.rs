// Given a list of integers, use a vector and return the median (when sorted,
// the value in the middle position) and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn main() {
  let mut numbers = vec![3, 5, 1, 23, 5, 67, 8, 1, 5];
  numbers.sort(); // [1, 1, 3, 5, 5, 5, 8, 23, 67];

  let mut averages: HashMap<i32, u8> = HashMap::new();

  let mut median = 0;

  for (i, number) in numbers.iter().enumerate() {
    let count = averages.entry(*number).or_insert(0);
    *count += 1;

    if median == 0 && i * 2 >= numbers.len() {
      median = *number;
    }
  }

  let mut mode: (i32, u8) = (0, 0);
  for (number, count) in averages {
    println!("{number}: {count}");
    if count > mode.1 {
      mode.0 = number;
      mode.1 = count;
    }
  }

  println!("Mode: {}", mode.0);
  println!("Median: {median}");
}
