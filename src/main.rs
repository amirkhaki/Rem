use std::fs::File;

use rem::lexer::Lexer;

fn main() -> std::io::Result<()> {
    let f = File::open("test.c")?;
    let tokens = Lexer::from(f);
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
