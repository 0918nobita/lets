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
