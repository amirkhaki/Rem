
use std::collections::{HashMap, HashSet};

use rem::parser::{Grammar, Symbol, Terminal};

fn print_hashmap(set: HashMap<Symbol, HashSet<Terminal>>) {
    for (k, v) in set {
        match k {
            rem::parser::Symbol::Terminal(t) => match t {
                rem::parser::Terminal::Token(t) => {
                    if let rem::lexer::Token::String(s) = t {
                        print!("{s:?}")
                    } else {
                        print!("{t:?}")
                    }
                }
                rem::parser::Terminal::Epsilon => print!("Ep30lon"),
            },
            rem::parser::Symbol::NonTerminal(t) => print!("{t:?}"),
        }
        print!(" => ");
        for vv in v {
            match vv {
                rem::parser::Terminal::Token(t) => {
                    if let rem::lexer::Token::String(s) = t {
                        print!("{s:?}, ")
                    } else {
                        print!("{t:?}, ")
                    }
                }
                rem::parser::Terminal::Epsilon => print!("Ep30lon, "),
            };
        }
        println!();
    }
}
fn main() -> std::io::Result<()> {
    // let f = File::open("test.c")?;
    // let tokens = Lexer::from(f);
    // for token in tokens {
    //     println!("{:?}", token);
    // }
    let grammar = Grammar::from_stdin();
    println!("printing first set of the grammmar");
    let first_set = grammar.first_set();
    print_hashmap(first_set);
    println!("======================================");
    println!("printing follow set of the grammmar");
    let follow_set = grammar.follow_set();
    print_hashmap(follow_set);
    Ok(())
}
