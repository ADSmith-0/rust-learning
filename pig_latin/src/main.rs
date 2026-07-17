fn main() {
  println!("{}", to_pig_latin("hello world how are you".to_string()));
}

fn to_pig_latin(str: String) -> String {
  let vowels = ['a', 'e', 'i', 'o', 'u'];
  let mut pig_latin = String::from("");

  for word in str.split_whitespace() {
    if vowels.contains(&word.chars().next().unwrap()) {
      pig_latin.push_str(&format!("{word}hay "));
      continue;
    }

    pig_latin.push_str(&format!(
      "{}{}ay ",
      &word[1..],
      word.chars().next().unwrap()
    ));
  }

  pig_latin
}
