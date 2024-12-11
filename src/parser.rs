use std::{
    collections::{HashMap, HashSet},
    io,
};

use crate::lexer::Token;

type NonTerminal = String;
#[derive(PartialEq, Clone, Eq, Hash, Debug)]
pub enum Terminal {
    Token(Token),
    Epsilon,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}
type Production = Vec<Symbol>;

#[derive(Default)]
pub struct Grammar {
    start: NonTerminal,
    rules: HashMap<NonTerminal, Vec<Production>>,
    terminals: HashSet<Terminal>,
    non_terminals: HashSet<NonTerminal>,
}

impl Grammar {
    pub fn from_stdin() -> Self {
        let mut grammar: Self = Default::default();
        println!("enter rules as SYMBOL => terminal + NONTERMINAL");
        println!("enter END to end getting rules");
        let mut input = String::from("start");
        loop {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            input = input.trim().to_string();
            if input == "END" {
                break;
            }
            let mut parts = input.split("=>");
            let symbol = parts.next().expect("invalid syntax").trim().to_string();
            let rest = parts.next().expect("invalid syntax").trim().to_string();
            let parts = rest.split_whitespace();
            let mut rest: Production = vec![];
            for part in parts {
                if part.to_string().chars().all(char::is_alphabetic) && part == part.to_uppercase()
                {
                    rest.push(Symbol::NonTerminal(part.to_string()));
                    grammar.non_terminals.insert(part.to_string());
                } else {
                    if part == "ep30" {
                        rest.push(Symbol::Terminal(Terminal::Epsilon));
                        grammar.terminals.insert(Terminal::Epsilon);
                    } else {
                        let token = Terminal::Token(Token::String(part.to_string()));
                        rest.push(Symbol::Terminal(token.clone()));
                        grammar.terminals.insert(token);
                    }
                }
            }
            grammar.rules.entry(symbol.clone()).or_default().push(rest);
            grammar.non_terminals.insert(symbol);
        }
        loop {
            println!("set start symbol");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            input = input.trim().to_string();
            if grammar.non_terminals.contains(&input) {
                grammar.start = input;
                break;
            }
            print!("your nonterminals are: ");
            for v in &grammar.non_terminals {
                print!("{v} ");
            }
            println!();
        }

        grammar
    }
    pub fn follow_set(&self) -> HashMap<Symbol, HashSet<Terminal>> {
        let mut follow: HashMap<Symbol, HashSet<Terminal>> = Default::default();
        let mut first = self.first_set();
        for nt in &self.non_terminals {
            let s = Symbol::NonTerminal(nt.clone());
            follow.entry(s).or_default();
        }
        let s = Symbol::NonTerminal(self.start.clone());
        follow
            .entry(s)
            .or_default()
            .insert(Terminal::Token(Token::EOF));
        let mut changing = true;
        while changing {
            changing = false;
            for (key, value) in &self.rules {
                for rule in value {
                    let s = Symbol::NonTerminal(key.clone());
                    let mut trailer = follow.entry(s).or_default().clone();
                    for b in rule.iter().rev() {
                        match b {
                            Symbol::Terminal(t) => {
                                trailer = Default::default();
                                trailer.insert(t.clone());
                            }
                            Symbol::NonTerminal(_nt) => {
                                for symbol in trailer.iter() {
                                    let follow_b = follow.entry(b.clone()).or_default();
                                    changing = follow_b.insert(symbol.clone()) || changing;
                                }
                                let mut first_b = first.entry(b.clone()).or_default().clone();
                                if first_b.contains(&Terminal::Epsilon) {
                                    first_b.remove(&Terminal::Epsilon);
                                    trailer.extend(first_b);
                                } else {
                                    trailer = first_b;
                                }
                            }
                        }
                    }
                }
            }
        }
        follow
    }
    pub fn first_set(&self) -> HashMap<Symbol, HashSet<Terminal>> {
        let mut first: HashMap<Symbol, HashSet<Terminal>> = Default::default();
        for t in &self.terminals {
            let s = Symbol::Terminal(t.clone());
            first.entry(s).or_default().insert(t.clone());
        }
        for nt in &self.non_terminals {
            let s = Symbol::NonTerminal(nt.clone());
            first.entry(s).or_default();
        }
        let mut changing = true;
        while changing {
            changing = false;
            for (key, value) in &self.rules {
                for rule in value {
                    // let s = Symbol::NonTerminal(key.clone());
                    let s = rule.first().unwrap().clone();
                    let mut rhs = first.entry(s).or_default().clone();
                    rhs.remove(&Terminal::Epsilon);
                    let mut trailing = true;
                    for (pos, v) in rule[..rule.len() - 1].iter().enumerate() {
                        if first
                            .entry(v.clone())
                            .or_default()
                            .contains(&Terminal::Epsilon)
                        {
                            let mut left = first.entry(rule[pos + 1].clone()).or_default().clone();
                            left.remove(&Terminal::Epsilon);
                            for symbol in left {
                                rhs.insert(symbol);
                            }
                        } else {
                            trailing = false;
                            break;
                        }
                    }
                    let last = rule.last().unwrap().clone();
                    let last = first.entry(last).or_default().clone();
                    if trailing && last.contains(&Terminal::Epsilon) {
                        rhs.insert(Terminal::Epsilon);
                    }
                    let s = Symbol::NonTerminal(key.clone());
                    let key_first = first.entry(s).or_default();
                    for symbol in rhs {
                        changing = key_first.insert(symbol) || changing;
                    }
                }
            }
        }

        first
    }
}
