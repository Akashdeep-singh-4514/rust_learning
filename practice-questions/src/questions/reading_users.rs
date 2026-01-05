use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs;

use crate::questions::utils::inputs;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct User {
    id: usize,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Company {
    name: String,
    #[serde(rename = "catchPhrase")]
    catch_phrase: String,
    bs: String,
}

#[derive(Debug, Clone)]
struct Users {
    val: Box<[User]>,
}

#[allow(unused)]
impl Users {
    fn new() -> Self {
        Self { val: Box::new([]) }
    }

    fn length(&self) -> usize {
        self.val.len()
    }

    fn list(&self, fields: &[String], count: usize) {
        let mut printed = 0;
        for user in self.val.iter() {
            if count > 0 && printed >= count {
                break;
            }

            if fields.is_empty() {
                // print all fields
                println!(
                    "ID: {}, Name: {}, Username: {}, Email: {}, Phone: {}, Website: {}, Company: {}",
                    user.id,
                    user.name,
                    user.username,
                    user.email,
                    user.phone,
                    user.website,
                    user.company.name
                );
                println!(
                    "Address: {}, {}, {}, {}, Geo: ({}, {})",
                    user.address.street,
                    user.address.suite,
                    user.address.city,
                    user.address.zipcode,
                    user.address.geo.lat,
                    user.address.geo.lng
                );
                println!("----------------------------------------------------");
            } else {
                let mut out = Vec::new();
                for field in fields {
                    let value = match field.as_str() {
                        "id" => user.id.to_string(),
                        "name" => user.name.clone(),
                        "username" => user.username.clone(),
                        "email" => user.email.clone(),
                        "phone" => user.phone.clone(),
                        "website" => user.website.clone(),
                        "company" => user.company.name.clone(),
                        "street" => user.address.street.clone(),
                        "suite" => user.address.suite.clone(),
                        "city" => user.address.city.clone(),
                        "zipcode" => user.address.zipcode.clone(),
                        "lat" => user.address.geo.lat.clone(),
                        "lng" => user.address.geo.lng.clone(),
                        _ => format!("Unknown field: {}", field),
                    };
                    out.push(format!("{}: {}", field, value));
                }
                println!("{}", out.join(", "));
            }

            printed += 1;
        }
    }

    fn add(&mut self, user: User) {
        let mut vec = self.val.to_vec();
        vec.push(user);
        self.val = vec.into_boxed_slice();
    }

    fn remove(&mut self, id: usize) -> bool {
        let original_len = self.val.len();

        let new_data: Vec<User> = self
            .val
            .iter()
            .cloned()
            .filter(|user| user.id != id)
            .collect();

        if new_data.len() == original_len {
            return false;
        }

        self.val = new_data.into_boxed_slice();
        true
    }

    fn read_from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(file_path)?;
        let parsed: Vec<User> = serde_json::from_str(&contents)?;
        Ok(Users {
            val: parsed.into_boxed_slice(),
        })
    }

    fn write_to_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(&self.val)?;
        fs::write(file_path, json)?;
        Ok(())
    }
    fn next_id(&self) -> usize {
        self.val.iter().map(|u| u.id).max().unwrap_or(0) + 1
    }
}

fn help() {
    println!("==========================================================");
    println!("Users CLI - Commands Guide");
    println!("add                       : Start entering new user data");
    println!("list                      : List all users with all fields");
    println!("list -f <field1,field2>  : List users with selected fields");
    println!("list -f <fields> -n <n>   : List first n users with selected fields");
    println!("remove <id>               : Remove user by ID");
    println!("write                     : Save changes to file");
    println!("size                      : Print total number of users");
    println!("help                      : Show this guide");
    println!("exit                      : Exit the CLI");
    println!("==========================================================");
}
fn parse_list_flags(words: &[&str]) -> (Vec<String>, usize) {
    let mut fields: Vec<String> = Vec::new();
    let mut count: usize = 0;

    let mut i = 1; // skip the "list" command itself
    while i < words.len() {
        match words[i] {
            "-f" => {
                i += 1;
                while i < words.len() && words[i] != "-n" {
                    fields.push(words[i].to_lowercase());
                    i += 1;
                }
                continue; // skip outer i+=1 for inner loop
            }
            "-n" => {
                i += 1;
                if i < words.len() {
                    count = words[i].parse().unwrap_or(0);
                }
            }
            _ => {}
        }
        i += 1;
    }

    (fields, count)
}

#[allow(unused)]
const USERS_FILE_PATH: &str = "src/questions/utils/users.json";
fn add_handler(users: &mut Users) {
    let sample_user = User {
        id: users.next_id(),
        name: "John Doe".to_string(),
        username: "johnd".to_string(),
        email: "john@example.com".to_string(),
        phone: "999-888-7777".to_string(),
        website: "johndoe.dev".to_string(),
        address: Address {
            street: "Main Street".to_string(),
            suite: "Apt 101".to_string(),
            city: "Delhi".to_string(),
            zipcode: "110001".to_string(),
            geo: Geo {
                lat: "28.6139".to_string(),
                lng: "77.2090".to_string(),
            },
        },
        company: Company {
            name: "Acme Corp".to_string(),
            catch_phrase: "We build things".to_string(),
            bs: "software solutions".to_string(),
        },
    };

    users.add(sample_user);
    println!("Sample user added successfully.");
}

#[allow(unused)]
pub fn start() {
    let mut users = match Users::read_from_file(USERS_FILE_PATH) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Failed to load users: {}", e);
            return;
        }
    };

    println!("start with users-cli, type 'help' to see guide");

    loop {
        print!("$ ");
        let input = inputs::str_inputs().replace("\n", "");
        let words: Vec<&str> = input.split_whitespace().collect();

        if words.is_empty() {
            continue;
        }

        match words[0] {
            "help" => help(),
            "exit" => break,
            "size" => println!("Total users: {}", users.length()),

            "list" => {
                let (fields, count) = parse_list_flags(&words);
                users.list(&fields, count);
            }

            "add" => add_handler(&mut users),
            "remove" => {
                if words.len() != 2 {
                    println!("Usage: remove <id>");
                } else {
                    match words[1].parse::<usize>() {
                        Ok(id) => {
                            if users.remove(id) {
                                println!("Removed user {}", id);
                            } else {
                                println!("Invalid id");
                            }
                        }
                        Err(_) => println!("id must be a number"),
                    }
                }
            }
            "write" => match users.write_to_file(USERS_FILE_PATH) {
                Ok(_) => println!("Users saved successfully."),
                Err(e) => {
                    eprintln!("Failed to write users file.");
                    eprintln!("Reason: {}", e);
                }
            },

            _ => println!("Type 'help' to see guide"),
        }
    }
}
