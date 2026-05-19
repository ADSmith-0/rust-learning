fn main() {
  loop {
    let mut input = String::new();

    std::io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let index = input.len() - 2;

    let degree = &input[..index];
    let degree: f64 = match degree.trim().parse() {
      Ok(v) => v,
      Err(e) => {
        println!("Could not convert value: {e}");
        continue;
      }
    };

    let unit = &input[index..].as_bytes();

    if unit[0] == b'f' {
      println!("Celsius: {}", f_to_c(degree));
    } else {
      println!("Fahrenheit: {}", c_to_f(degree));
    }
  }
}

fn f_to_c(f: f64) -> f64 {
  (f - 32.0) / 1.8
}

fn c_to_f(c: f64) -> f64 {
  (c * 1.8) + 32.0
}
