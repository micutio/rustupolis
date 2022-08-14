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

mod lexing;

use std::io;
use std::io::Write;

use futures::executor;
use lexing::Lexer;
use rustupolis::space::Space;
use rustupolis::store::SimpleStore;
use rustupolis::tuple::Tuple;

fn main() {
    println!("Rustupolis CLI");

    let mut cli = Cli::new(io::stdin(), io::stdout());
    cli.run()
}

#[allow(clippy::upper_case_acronyms)]
enum RequiredAction {
    CLOSE,
    DETACH,
    NONE,
}

/// The CLI wraps the tuple space into an application, allowing the user to insert, query and
/// retrieve data in tuple form.
/// For now this is a simple echo server that takes tuples and prints results for any queries.
/// Future versions are planned to include persistent sessions (file or daemon-based) and complete
/// support for asynchronous operations.
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
    /// Ideas for more pre-defined commands:
    ///
    /// - `attach` - re-connect to a running tuple space session
    /// - `detach` - close the CLI, but keep the tuple space server running in the background
    ///
    // TODO: Keep the list updated.
    fn process_input(&mut self, input: &str) -> RequiredAction {
        use self::RequiredAction::*;
        println!("user echo: {}", input);
        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.is_empty() {
            return NONE;
        }

        let command = tokens.first();
        match command {
            Some(&"create") => self.cmd_create(&tokens[1..]),
            Some(&"close") => self.cmd_close(),
            Some(&"detach") => self.cmd_detach(),
            Some(&"out") => self.cmd_tuple_out(&tokens[1..]),
            Some(&"read") | Some(&"rd") => self.cmd_tuple_read(&tokens[1..]),
            Some(&"take") | Some(&"in") => self.cmd_tuple_take(&tokens[1..]),
            _ => {
                println!("unknown command");
                NONE
            }
        }
    }

    /// Create a new tuple space.
    /// In future versions this should take parameters to control the variation and underlying
    /// server attributes.
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

    fn cmd_tuple_out(&mut self, parameters: &[&str]) -> RequiredAction {
        if let Some(space) = &mut self.tuplespace {
            let param_list = parameters.join(" ");
            let tuples: Vec<Tuple> = Lexer::new(&param_list).collect();
            for t in tuples {
                if !t.is_empty() {
                    if t.is_defined() {
                        if let Err(e) = executor::block_on(space.tuple_out(t)) {
                            eprintln!("Cannot push tuple into space! Encountered error {:?}", e);
                        } else {
                            println!("pushed tuple(s) {} into tuple space", param_list);
                        }
                    } else {
                        eprintln!("Cannot push tuple into space! The given tuple is ill-defined.");
                    }
                }
            }
        } else {
            println!("Cannot push tuple into space! There is no tuple space initialised");
        }
        RequiredAction::NONE
    }

    fn cmd_tuple_read(&mut self, parameters: &[&str]) -> RequiredAction {
        if let Some(space) = &mut self.tuplespace {
            let param_list = parameters.join(" ");
            let tuples: Vec<Tuple> = Lexer::new(&param_list).collect();
            for rd_tup in tuples {
                if !rd_tup.is_empty() {
                    println!("reading tuple matching {} from space", rd_tup);
                    if let Some(match_tup) = executor::block_on(space.tuple_rd(rd_tup)) {
                        if match_tup.is_empty() {
                            eprintln!("No matching tuple could be found.");
                        } else {
                            println!("found match: {}", match_tup);
                        }
                    }
                }
            }
        } else {
            println!("Cannot read tuple from space! There is no tuple space initialised");
        }
        // TODO: This suffices for our echo test cli. In the future this should return a tuple!
        RequiredAction::NONE
    }

    fn cmd_tuple_take(&mut self, parameters: &[&str]) -> RequiredAction {
        if let Some(space) = &mut self.tuplespace {
            let param_list = parameters.join(" ");
            let tuples: Vec<Tuple> = Lexer::new(&param_list).collect();
            for rd_tup in tuples {
                if !rd_tup.is_empty() {
                    println!("pulling in tuple matching {} from space", rd_tup);
                    if let Some(match_tup) = executor::block_on(space.tuple_in(rd_tup)) {
                        if match_tup.is_empty() {
                            eprintln!("No matching tuple could be found.");
                        } else {
                            println!("found match: {}", match_tup);
                        }
                    }
                }
            }
        } else {
            println!("Cannot pull in tuple from space! There is no tuple space initialised");
        }
        // TODO: This suffices for our echo test cli. In the future this should return a tuple!
        RequiredAction::NONE
    }
}
