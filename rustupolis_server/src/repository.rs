use crate::lexing::Lexer;
use crate::repository::RequestResponse::{DataResponse, NoResponse, OkResponse, SpaceResponse};
use futures::executor;
use rustupolis::space::Space;
use rustupolis::store::SimpleStore;
use rustupolis::tuple::Tuple;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

pub struct Repository {
    tuple_spaces: Arc<RwLock<HashMap<String, Arc<Mutex<Space<SimpleStore>>>>>>,
}

pub enum RequestResponse {
    SpaceResponse(Arc<Mutex<Space<SimpleStore>>>),
    DataResponse(Tuple),
    OkResponse(),
    NoResponse(String),
}

impl Repository {
    pub fn new() -> Repository {
        Repository {
            tuple_spaces: Arc::new(RwLock::new(HashMap::with_capacity(128))),
        }
    }

    pub fn add_tuple_space(&self, name: String) {
        self.tuple_spaces
            .write()
            .unwrap()
            .insert(name, Arc::new(Mutex::new(Space::new(SimpleStore::new()))));
    }

    pub fn manage_request(
        &self,
        request: String,
        tuple_space: Option<&Arc<Mutex<Space<SimpleStore>>>>,
    ) -> RequestResponse {
        let words: Vec<&str> = request.split_whitespace().collect();
        if words.len() != 0 {
            match words[0] {
                "create" => {
                    self.add_tuple_space(String::from(words[1]));
                    OkResponse()
                }
                "attach" => {
                    let tuple_spaces = self.tuple_spaces.read().unwrap();
                    let tuple_space_found = tuple_spaces.get(words[1]);
                    match tuple_space_found {
                        None => NoResponse(String::from("Tuple not found")),
                        Some(tuple_space_ref) => SpaceResponse(tuple_space_ref.clone()),
                    }
                }
                "out" => {
                    if let Some(x) = tuple_space {
                        let param_list = words[1..].join(" ");
                        let tuples: Vec<Tuple> = Lexer::new(&param_list).collect();

                        for t in tuples {
                            if !t.is_empty() {
                                if t.is_defined() {
                                    let mut space = x.lock().unwrap();

                                    if let Err(e) = executor::block_on(space.tuple_out(t)) {
                                        eprintln!(
                                            "Cannot push tuple into space! Encountered error {:?}",
                                            e
                                        );
                                    } else {
                                        println!("pushed tuple(s) {} into tuple space", param_list);
                                    }
                                } else {
                                    eprintln!("Cannot push tuple into space! The given tuple is ill-defined.");
                                }
                            }
                        }
                        OkResponse()
                    } else {
                        NoResponse(String::from("Tuple space not found\n"))
                    }
                }
                "read" => {
                    if let Some(x) = tuple_space {
                        let param_list = words[1..].join(" ");
                        let tuples: Vec<Tuple> = Lexer::new(&param_list).collect();
                        let mut response: RequestResponse =
                            NoResponse(String::from("Somethings went wrong\n"));
                        for rd_tup in tuples {
                            if !rd_tup.is_empty() {
                                let mut space = x.lock().unwrap();
                                if let Some(match_tup) = executor::block_on(space.tuple_rd(rd_tup))
                                {
                                    if match_tup.is_empty() {
                                        response = NoResponse(String::from(
                                            "No matching tuple could be found.\n",
                                        ));
                                    } else {
                                        println!("reading tuples {} from space", match_tup);
                                        response = DataResponse(match_tup);
                                    }
                                }
                            }
                        }
                        response
                    } else {
                        NoResponse(String::from("Tuple space not found"))
                    }
                }
                "in" => {
                    if let Some(x) = tuple_space {
                        let param_list = words[1..].join(" ");
                        let tuples: Vec<Tuple> = Lexer::new(&param_list).collect();
                        let mut response: RequestResponse =
                            NoResponse(String::from("Somethings went wrong\n"));
                        for rd_tup in tuples {
                            if !rd_tup.is_empty() {
                                let mut space = x.lock().unwrap();
                                println!("pulling in tuple matching {} from space", rd_tup);
                                if let Some(match_tup) = executor::block_on(space.tuple_in(rd_tup))
                                {
                                    if match_tup.is_empty() {
                                        response = NoResponse(String::from(
                                            "No matching tuple could be found.\n",
                                        ));
                                    } else {
                                        response = DataResponse(match_tup);
                                    }
                                }
                            }
                        }
                        response
                    } else {
                        NoResponse(String::from("Tuple space not found"))
                    }
                }
                _ => NoResponse(String::from("Request doesn't exist")),
            }
        } else {
            NoResponse(String::from("Empty request"))
        }
    }
}
