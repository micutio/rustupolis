//! # Rustupolis CLI
//!
//! A tuple space client implementation.
//! Ultimately this will work offline as a self-sufficient tuple space server as well as by
//! connecting to a remote tuple space server.
//!

// TODO list:
//  - input parsing loop
//  - processing of parsed commands

#[macro_use]
extern crate rustupolis;

use std::io;
use std::io::Write;

use rustupolis::space::Space;
use rustupolis::store::SimpleStore;
use rustupolis::tuple::E;

fn main() {
    println!("Rustupolis CLI");

    let mut cli = Cli::new(io::stdin(), io::stdout());
    cli.run()
}

enum RequiredAction {
    CLOSE,
    DETACH,
    NONE,
}

struct Cli {
    stdin: io::Stdin,
    stdout: io::Stdout,
    tuplespace: Option<Space<SimpleStore>>,
}

impl Cli {
    fn new(stdin: io::Stdin, stdout: io::Stdout) -> Cli {
        Cli {
            stdin,
            stdout,
            tuplespace: None,
        }
    }

    fn run(&mut self) {
        use self::RequiredAction::*;
        let mut input = String::new();
        loop {
            print!("> ");
            self.stdout.flush().expect("failed to flush stdout");
            self.stdin
                .read_line(&mut input)
                .expect("failed to read input");
            let required_action = self.process_input(input.trim());
            // reset input
            input.clear();
            // TODO: implement proper actions
            match required_action {
                CLOSE => break,
                DETACH => break,
                NONE => {}
            }
        }
    }

    /// User input should always consist of a pre-defined command and user-defined parameters,
    /// separated by whitespaces.
    ///
    /// Ideas for pre-defined commands:
    ///
    /// - `create` - create new tuple space \ tuple space server
    /// - `close` or `quit` - tear down the tuple space and terminate the program
    /// - `detach` - close the CLI, but keep the tuple space server running in the background
    /// - `out <tuple>` - push the given <tuple> out into space
    ///
    // TODO: Keep the list updated.
    fn process_input(&mut self, input: &str) -> RequiredAction {
        use self::RequiredAction::*;
        println!("user echo: {}", input);
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();
        if tokens.is_empty() {
            return NONE;
        }

        let command = tokens.get(0);
        match command {
            Some(&"create") => self.cmd_create(&tokens[1..]),
            Some(&"close") => self.cmd_close(),
            Some(&"detach") => self.cmd_detach(),
            _ => {
                println!("unknown command");
                NONE
            }
        }
    }

    fn cmd_create(&mut self, parameters: &[&str]) -> RequiredAction {
        println!("creation parameters:");
        for p in parameters {
            println!("{}", p)
        }

        if self.tuplespace.is_none() {
            println!("creating new tuplespace");
            self.tuplespace = Some(Space::new(SimpleStore::new()));
        } else {
            println!("cannot create new tuple space! already exists");
        }
        RequiredAction::NONE
    }

    fn cmd_close(&mut self) -> RequiredAction {
        RequiredAction::CLOSE
    }

    fn cmd_detach(&mut self) -> RequiredAction {
        if self.tuplespace.is_none() {
            println!("Cannot detach! Error: no tuple space initialised");
            return RequiredAction::NONE;
        }
        RequiredAction::DETACH
    }
}
