use crate::client::Client;
use crate::constant::{ADMIN_ATTRIBUTE, ATTACH, CREATE, DELETE, EMPTY_REQUEST, IN, NO_MATCHING_TUPLE_FOUND, NO_PERMISSION, NO_TUPLE_SPACE_ATTACHED, OUT, PERMISSION, READ, REQUEST_DOESNT_EXIST, TUPLE_IS_EMPTY, TUPLE_SPACE_NOT_FOUND};
use crate::lexing::Lexer;
use crate::repository::RequestResponse::{DataResponse, NoResponse, OkResponse, SpaceResponse};
use futures::executor;
use rustupolis::space::Space;
use rustupolis::store::SimpleStore;
use rustupolis::tuple;
use rustupolis::tuple::{Tuple, E};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

pub struct Repository {
    tuple_spaces:           Arc<RwLock<HashMap<String, Arc<Mutex<Space<SimpleStore>>>>>>,
    permission_tuple_space: Arc<Mutex<Space<SimpleStore>>>,
}

pub enum RequestResponse {
    SpaceResponse(Client),
    DataResponse(Vec<Tuple>),
    OkResponse(),
    NoResponse(String),
}

impl Repository {
    pub fn new() -> Repository {
        let permission = Arc::new(Mutex::new(Space::new(SimpleStore::new())));
        let new_repository = Repository {
            tuple_spaces:           Arc::new(RwLock::new(HashMap::with_capacity(128))),
            permission_tuple_space: permission.clone(),
        };
        new_repository
            .tuple_spaces
            .write()
            .unwrap()
            .insert(String::from(PERMISSION), permission);
        let mut permission_tuple_space = new_repository.permission_tuple_space.lock().unwrap();
        let result = executor::block_on(permission_tuple_space.tuple_out(tuple!(
            E::str(CREATE),
            E::T(tuple!(E::str(ADMIN_ATTRIBUTE)))
        )));
        drop(permission_tuple_space);
        new_repository.add_permission_list(vec![String::from(ADMIN_ATTRIBUTE)], PERMISSION);
        match result {
            Ok(_) => new_repository,
            Err(error) => {
                panic!("{}", error)
            }
        }
    }

    pub fn add_tuple_space(&self, name: String) {
        self.tuple_spaces
            .write()
            .unwrap()
            .insert(name, Arc::new(Mutex::new(Space::new(SimpleStore::new()))));
    }

    pub fn remove_tuple_space(&self, name: &str) {
        self.tuple_spaces.write().unwrap().remove(name);
    }

    pub fn check_permission(
        &self,
        action: &str,
        attributes: &Vec<String>,
        tuple_space_name: Option<&str>,
    ) -> bool {
        let mut permission_space = self.permission_tuple_space.lock().unwrap();
        return match action {
            CREATE => {
                match executor::block_on(permission_space.tuple_rd(tuple!(E::str(action), E::Any)))
                {
                    None => false,
                    Some(tuple) => {
                        if tuple.is_empty() {
                            return false;
                        }
                        let rest = tuple.rest();
                        Repository::compare_attributes(rest.first(), attributes)
                    }
                }
            }
            _ => {
                match executor::block_on(permission_space.tuple_rd(tuple!(
                    E::str(tuple_space_name.unwrap()),
                    E::str(action),
                    E::Any
                ))) {
                    None => false,
                    Some(tuple) => {
                        if tuple.is_empty() {
                            return false;
                        }
                        let rest = tuple.rest().rest();
                        Repository::compare_attributes(rest.first(), attributes)
                    }
                }
            }
        };
    }

    fn compare_attributes(attributes_permission: &E, attributes_client: &Vec<String>) -> bool {
        if let E::T(tuple) = attributes_permission {
            let mut attributes_permission_list = Vec::with_capacity(156);
            if let E::S(attribute) = tuple.first() {
                attributes_permission_list.push(String::from(attribute));
            }
            while !tuple.rest().is_empty() {
                if let E::S(attribute) = tuple.first() {
                    attributes_permission_list.push(String::from(attribute));
                }
            }

            if attributes_client
                .iter()
                .filter(|&x| attributes_permission_list.contains(&x))
                .count()
                > 0
            {
                return true;
            }
            return false;
        }
        return false;
    }

    pub fn add_permission_list(&self, attributes: Vec<String>, tuple_space_name: &str) {
        if attributes.len() == 1 {
            let attribute = attributes.first().unwrap();
            self.add_permission(attribute, DELETE, tuple_space_name);
            self.add_permission(attribute, READ, tuple_space_name);
            self.add_permission(attribute, IN, tuple_space_name);
            self.add_permission(attribute, OUT, tuple_space_name);
        } else if attributes.len() == 4 {
            self.add_permission(&attributes[0], READ, tuple_space_name);
            self.add_permission(&attributes[1], IN, tuple_space_name);
            self.add_permission(&attributes[2], OUT, tuple_space_name);
            self.add_permission(&attributes[3], DELETE, tuple_space_name);
        }
    }

    pub fn add_permission(&self, attribute: &String, action: &str, tuple_space_name: &str) {
        let mut permission_space = self.permission_tuple_space.lock().unwrap();
        match executor::block_on(permission_space.tuple_out(tuple!(
            E::str(tuple_space_name),
            E::str(action),
            E::T(tuple!(E::S(attribute.clone())))
        ))) {
            Ok(_) => {}
            Err(error) => {
                println!("{}", error)
            }
        }
    }

    pub fn manage_request(
        &self,
        request: String,
        client_option: Option<&Client>,
    ) -> RequestResponse {
        let words: Vec<&str> = request.split_whitespace().collect();
        if words.len() != 0 {
            match words[0] {
                CREATE => {
                    let attribute_to_create = String::from(words[1]);
                    if self.check_permission(CREATE, &vec![attribute_to_create], None) {
                        self.add_tuple_space(String::from(words[2]));
                        let mut attributes_list: Vec<String> = Vec::with_capacity(126);
                        for index in 3..words.len() {
                            attributes_list.push(String::from(words[index]));
                        }
                        self.add_permission_list(attributes_list, words[2]);
                        OkResponse()
                    } else {
                        NoResponse(String::from(NO_PERMISSION))
                    }
                }
                DELETE => {
                    let attribute_to_delete = String::from(words[1]);
                    // TODO check attributes
                    if self.check_permission(DELETE, &vec![attribute_to_delete], Some(words[2])) {
                        self.remove_tuple_space(words[2]);
                        OkResponse()
                    } else {
                        NoResponse(String::from(NO_PERMISSION))
                    }
                }
                ATTACH => {
                    let tuple_spaces = self.tuple_spaces.read().unwrap();
                    let tuple_space_found = tuple_spaces.get(words[1]);
                    match tuple_space_found {
                        None => NoResponse(String::from(TUPLE_SPACE_NOT_FOUND)),
                        Some(tuple_space_ref) => {
                            let mut attributes_list: Vec<String> = Vec::with_capacity(126);
                            for index in 2..words.len() {
                                attributes_list.push(String::from(words[index]));
                            }
                            SpaceResponse(Client::new(
                                tuple_space_ref.clone(),
                                attributes_list,
                                words[1],
                            ))
                        }
                    }
                }
                OUT => {
                    if let Some(client) = client_option {
                        if self.check_permission(
                            OUT,
                            client.attributes(),
                            Some(client.tuple_space_name()),
                        ) {
                            let param_list = words[1..].join(" ");
                            let tuple_list: Vec<Tuple> = Lexer::new(&param_list).collect();

                            for tuple in tuple_list {
                                if !tuple.is_empty() {
                                    if tuple.is_defined() {
                                        let mut space = client.tuple_space().lock().unwrap();

                                        if let Err(error) =
                                            executor::block_on(space.tuple_out(tuple))
                                        {
                                            eprintln!(
                                                "Cannot push tuple into space! Encountered error {:?}",
                                                error
                                            );
                                        } else {
                                            println!(
                                                "pushed tuple(s) {} into tuple space",
                                                param_list
                                            );
                                        }
                                    } else {
                                        eprintln!("Cannot push tuple into space! The given tuple is ill-defined.");
                                    }
                                }
                            }
                            OkResponse()
                        } else {
                            NoResponse(String::from(NO_PERMISSION))
                        }
                    } else {
                        NoResponse(String::from(NO_TUPLE_SPACE_ATTACHED))
                    }
                }
                READ => {
                    if let Some(client) = client_option {
                        if self.check_permission(
                            READ,
                            client.attributes(),
                            Some(client.tuple_space_name()),
                        ) {
                            let param_list = words[1..].join(" ");
                            let tuples: Vec<Tuple> = Lexer::new(&param_list).collect();
                            let mut response: RequestResponse = NoResponse(String::from(""));
                            let mut tuples_result : Vec<Tuple> = Vec::with_capacity(124);
                            for rd_tup in tuples {
                                if !rd_tup.is_empty() {
                                    let mut space = client.tuple_space().lock().unwrap();
                                    if let Some(match_tup) =
                                        executor::block_on(space.tuple_rd(rd_tup))
                                    {
                                        if match_tup.is_empty() {
                                            response = NoResponse(String::from(NO_MATCHING_TUPLE_FOUND));
                                        } else {
                                            println!("reading tuples {} from space", match_tup);
                                            tuples_result.push(match_tup);
                                        }
                                    }
                                } else {
                                    response = NoResponse(String::from(TUPLE_IS_EMPTY));
                                }
                            }
                            if tuples_result.is_empty() {
                                response
                            }else {
                                DataResponse(tuples_result)
                            }

                        } else {
                            NoResponse(String::from(NO_PERMISSION))
                        }
                    } else {
                        NoResponse(String::from(NO_TUPLE_SPACE_ATTACHED))
                    }
                }
                IN => {
                    if let Some(client) = client_option {
                        if self.check_permission(
                            IN,
                            client.attributes(),
                            Some(client.tuple_space_name()),
                        ) {
                            let param_list = words[1..].join(" ");
                            let tuples: Vec<Tuple> = Lexer::new(&param_list).collect();
                            let mut response: RequestResponse = NoResponse(String::from(""));
                            let mut tuples_result : Vec<Tuple> = Vec::with_capacity(124);
                            for rd_tup in tuples {
                                if !rd_tup.is_empty() {
                                    let mut space = client.tuple_space().lock().unwrap();
                                    println!("pulling in tuple matching {} from space", rd_tup);
                                    if let Some(match_tup) =
                                        executor::block_on(space.tuple_in(rd_tup))
                                    {
                                        if match_tup.is_empty() {
                                            response = NoResponse(String::from(NO_MATCHING_TUPLE_FOUND));
                                        } else {
                                            tuples_result.push(match_tup);
                                        }
                                    }
                                } else {
                                    response = NoResponse(String::from(TUPLE_IS_EMPTY));
                                }
                            }
                            if tuples_result.is_empty() {
                                response
                            }else {
                                DataResponse(tuples_result)
                            }
                        } else {
                            NoResponse(String::from(NO_PERMISSION))
                        }
                    } else {
                        NoResponse(String::from(NO_TUPLE_SPACE_ATTACHED))
                    }
                }
                _ => NoResponse(String::from(REQUEST_DOESNT_EXIST)),
            }
        } else {
            NoResponse(String::from(EMPTY_REQUEST))
        }
    }
}
