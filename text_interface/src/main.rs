use std::collections::HashMap;

fn main() {
  let mut departments: HashMap<String, Vec<String>> = HashMap::new();

  let commands = ["add".to_string(), "list".to_string()];

  loop {
    let mut input = String::new();

    std::io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let words: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    if !commands.contains(&words[0].to_lowercase()) {
      println!("Command not found");
      continue;
    }

    if &words[0].to_lowercase() == &commands[0] {
      if words.len() < 4 || &words[2] != "to" {
        println!("Command format incorrect");
        continue;
      }

      let department = words[3].clone();
      let people = departments.entry(department).or_insert(Vec::new());

      let person = words[1].clone();
      people.push(person);
      continue;
    }

    if &words[0].to_lowercase() == &commands[1] {
      if words.len() < 2 {
        println!("Command format incorrect");
        continue;
      }

      let department = words[1].clone();
      let people = departments.entry(department.clone()).or_insert(Vec::new());

      println!("For {}", department);
      for person in people {
        println!("\t{person}");
      }
      continue;
    }
  }
}
