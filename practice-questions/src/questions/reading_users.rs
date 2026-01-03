use serde::Deserialize;
use std::error::Error;
use std::fs;

use crate::questions::utils::inputs;

#[derive(Debug, Deserialize)]
struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

#[derive(Debug, Deserialize)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Debug, Deserialize)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Debug, Deserialize)]
struct Company {
    name: String,
    #[serde(rename = "catchPhrase")]
    catch_phrase: String,
    bs: String,
}

#[derive(Debug)]
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

    fn read_from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(file_path)?;
        let parsed: Vec<User> = serde_json::from_str(&contents)?;
        Ok(Users {
            val: parsed.into_boxed_slice(),
        })
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

const USERS_FILE_PATH: &str = "src/questions/utils/users.json";

pub fn start() {
    let users = match Users::read_from_file(USERS_FILE_PATH) {
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

            "add" => println!("add"),
            "remove" => println!("remove"),
            "write" => println!("write"),
            _ => println!("Type 'help' to see guide"),
        }
    }
}
