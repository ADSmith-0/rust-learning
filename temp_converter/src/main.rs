fn main() {
  loop {
    let mut input = String::new();

    std::io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let input: f64 = match input.trim().parse() {
      Ok(v) => v,
      Err(e) => {
        println!("Could not convert value: {e}");
        continue;
      }
    };

    println!("Celsius: {}", f_to_c(input));
  }
}

fn f_to_c(f: f64) -> f64 {
  (f - 32.0) / 1.8
}

// fn c_to_f(c: f64) -> f64 {
//   (c * 1.8) + 32.0
// }
