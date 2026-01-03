use crate::questions::utils::inputs;

#[derive(Debug)]
struct List {
    data: Box<[String]>,
    size: usize,
}

impl List {
    fn new() -> Self {
        Self {
            data: Box::new([]),
            size: 0,
        }
    }

    fn push(&mut self, value: &str) {
        let mut new_data = Vec::with_capacity(self.size + 1);
        for v in self.data.iter() {
            new_data.push(v.clone());
        }
        new_data.push(value.to_string());
        self.data = new_data.into_boxed_slice();
        self.size += 1;
    }

    fn get(&self, index: usize) -> Option<&str> {
        if index < self.size {
            Some(&self.data[index])
        } else {
            None
        }
    }

    fn remove(&mut self, index: usize) -> bool {
        if index >= self.size {
            return false;
        }

        let mut new_data = Vec::with_capacity(self.size - 1);

        for (i, v) in self.data.iter().enumerate() {
            if i != index {
                new_data.push(v.clone());
            }
        }

        self.data = new_data.into_boxed_slice();
        self.size -= 1;
        true
    }

    fn display(&self) {
        for (i, value) in self.data.iter().enumerate() {
            println!("{}: {}", i, value);
        }
    }
}

fn help() {
    println!("==========================================================");
    println!("Guide to use todo-cli");
    println!("add <task>: add task");
    println!("list: list tasks");
    println!("remove <id>: remove task by index");
    println!("help: to see user guide");
    println!("exit: to exit");
    println!("==========================================================");
}

fn intro() {
    println!(">> start using todo-cli enter 'help' for guide")
}
#[allow(unused)]
pub fn start() {
    let mut list = List::new();
    intro();
    loop {
        print!("$ ");
        let input = inputs::str_inputs().replace("\n", "");
        let words: Vec<&str> = input.split_whitespace().collect();
        if words.len() >= 1 {
            match words[0] {
                "help" => help(),
                "exit" => break,
                "list" => list.display(),
                "add" => {
                    if words.len() < 2 {
                        println!("Usage: add <task>");
                    } else {
                        let task = words[1..].join(" ");
                        list.push(&task);
                        println!("Added: {}", task);
                    }
                }
                "get" => {
                    if words.len() != 2 {
                        println!("Usage: remove <id>");
                    } else {
                        match words[1].parse::<usize>() {
                            Ok(index) => match list.get(index) {
                                Some(task) => {
                                    println!("task at {} is {}", index, task);
                                }
                                None => {
                                    println!("Invalid index");
                                }
                            },
                            Err(_) => println!("Index must be a number"),
                        }
                    }
                }
                "remove" => {
                    if words.len() != 2 {
                        println!("Usage: remove <id>");
                    } else {
                        match words[1].parse::<usize>() {
                            Ok(index) => {
                                if list.remove(index) {
                                    println!("Removed task {}", index);
                                } else {
                                    println!("Invalid index");
                                }
                            }
                            Err(_) => println!("Index must be a number"),
                        }
                    }
                }
                _ => {
                    println!("type help to see guide")
                }
            }
        }
    }
}
