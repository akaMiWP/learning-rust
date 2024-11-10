fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    //println!("{}, {}", r1, r2);

    change(&mut s);

    let reference_to_nothing = dangle();

    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {word}");
}

fn change(some_string: &mut String) {
    &some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
