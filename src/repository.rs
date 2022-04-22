use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use futures::executor;
use rustupolis::space::Space;
use rustupolis::store::SimpleStore;
use rustupolis::tuple::{Tuple};
use crate::lexing::Lexer;
use crate::repository::RequestResponse::{DataResponse, NoResponse, OkResponse};

pub struct Repository{
    tuple_spaces: HashMap<String, Arc<Mutex<Space<SimpleStore>>>>,
}

pub enum RequestResponse<'a> {
    SpaceResponse(Option<&'a Arc<Mutex<Space<SimpleStore>>>>),
    DataResponse(Tuple),
    OkResponse(),
    NoResponse(String)
}

impl Repository {
    pub fn new() -> Repository {
        let mut r = Repository{
            tuple_spaces: HashMap::with_capacity(128)
        };
        r.tuple_spaces.insert("test".parse().unwrap(), Arc::new(Mutex::new(Space::new(SimpleStore::new()))));
        r
    }

    pub fn manage_request(&self, request:String, tuple_space: Option<&&Arc<Mutex<Space<SimpleStore>>>>) -> RequestResponse {
        let words : Vec<&str> = request.split_whitespace().collect();
        if words.len() != 0{
            match words[0] {
                "attach" => RequestResponse::SpaceResponse(self.tuple_spaces.get(words[1])),
                "out" => {
                    if let Some(x) = tuple_space {
                        let param_list = words[1..].join(" ");
                        let tuples: Vec<Tuple> = Lexer::new(&param_list).collect();

                        for t in tuples {
                            if !t.is_empty() {
                                if t.is_defined() {
                                    let mut space = x.lock().unwrap();

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
                        OkResponse()
                    }else {
                        NoResponse(String::from("Tuple space not found\n"))
                    }

                }
                "read" => {
                    if let Some(x) = tuple_space {
                        let param_list = words[1..].join(" ");
                        let tuples: Vec<Tuple> = Lexer::new(&param_list).collect();
                        let mut response: RequestResponse = NoResponse(String::from("Somethings went wrong\n"));
                        for rd_tup in tuples {
                            if !rd_tup.is_empty() {
                                let mut space = x.lock().unwrap();
                                println!("reading tuples {} from space", rd_tup);
                                if let Some(match_tup) = executor::block_on(space.tuple_rd(rd_tup)) {
                                    if match_tup.is_empty() {
                                        response = NoResponse(String::from("No matching tuple could be found.\n"));
                                    } else {
                                        response = DataResponse(match_tup);
                                    }
                                }
                            }
                        }
                        response
                    }else {
                        NoResponse(String::from("Tuple space not found"))
                    }
                }
                "in" => {
                    if let Some(x) = tuple_space {
                        let param_list = words[1..].join(" ");
                        let tuples: Vec<Tuple> = Lexer::new(&param_list).collect();
                        let mut response: RequestResponse = NoResponse(String::from("Somethings went wrong\n"));
                        for rd_tup in tuples {
                            if !rd_tup.is_empty() {
                                let mut space = x.lock().unwrap();
                                println!("pulling in tuple matching {} from space", rd_tup);
                                if let Some(match_tup) = executor::block_on(space.tuple_in(rd_tup)) {
                                    if match_tup.is_empty() {
                                        response = NoResponse(String::from("No matching tuple could be found.\n"));
                                    } else {
                                        response = DataResponse(match_tup);
                                    }
                                }
                            }
                        }
                        response
                    }else {
                        NoResponse(String::from("Tuple space not found"))
                    }
                }
                _ => NoResponse(String::from("Request doesn't exist")),
            }
        }else{
            NoResponse(String::from("Empty request"))
        }
    }

}