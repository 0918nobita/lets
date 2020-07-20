use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let word = first_word(&s);
    println!("{}", word);

    let num_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&num_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(&char_list));

    fs::create_dir_all("./.lets-cache")?;

    let mut file = File::create("./.lets-cache/build.ninja")?;
    writeln!(
        file,
        "rule hello\n  command = echo HELLO > $out\n\nbuild ../hello.txt: hello"
    )?;

    Command::new("ninja")
        .current_dir("./.lets-cache")
        .output()?;
    Ok(())
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
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

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
