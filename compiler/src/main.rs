use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    let mut file = File::create("./build.ninja")?;
    writeln!(
        file,
        "rule hello\n  command = echo \"HELLO\" > $out\n\nbuild hello.txt: hello"
    )?;
    Command::new("ninja").output()?;
    Ok(())
}
