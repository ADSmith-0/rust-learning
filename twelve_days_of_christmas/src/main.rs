fn main() {
  let days = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth",
    "eleventh", "twelfth",
  ];

  let gifts = [
    "A partridge in a pear tree",
    "Two turtle doves, and",
    "Three french hens,",
    "Four colley birds,",
    "Five gold rings,",
    "Six geese a a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,",
  ];

  for i in 0..=11 {
    println!(
      "On the {} day of Christmas my true love gave to me:",
      days[i]
    );

    for j in (0..=i).rev() {
      println!("{}", gifts[j]);
    }

    println!("");
  }
}
