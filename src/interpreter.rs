use std::io::stdin;

use rand::prelude::*;
use rand::seq::IndexedRandom;

use crate::rules::*;
use crate::parser::ThueProgram;

pub fn run_program (tp: ThueProgram) {
    let mut output = tp.input.clone();

    let mut rng = rand::rng();

    loop {
        let matching_rules = tp.rules.iter().filter(|it| output.contains(&it.left)).collect::<Vec<_>>();
        let Some(rule) = matching_rules.choose(&mut rng) else {
            break;
        };
        let mut repl = rule.right.clone();
        if rule.special == SpecialType::Output {
            // If the rule is empty, output a newline
            print!("{}", if rule.right != "" {
                rule.right.clone()
            } else { String::from("\n") });
            repl = String::from("");
        }
        if rule.special == SpecialType::Input {
            repl = readline()
        }
        output = output.replace(&rule.left, &repl);
    }

    println!("\nFinal string:\n{}", output);
}

// Gets a line from stdin
// https://users.rust-lang.org/t/how-to-get-user-input/5176/2
fn readline () -> String {
    println!("Input:");
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() { s.pop(); }
    if let Some('\r')=s.chars().next_back() { s.pop(); }
    s
}
