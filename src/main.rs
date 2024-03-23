#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

// add cli support
use std::env;

/* https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
 *
 * TODO week after
 * [X] build with nix
 * [X] add cli support
 * [X] add cli options and map them
 * CLI supper buggy atm and option parsing ist not really implemented
 * i just made a quick test while "sleeping"
 *      - [X] list tasks
 *      - [X] add task
 *      - [X] task done
 *      - [ ] move up
 *      - [ ] move down
 *
 * New thing i learned is that "///" are doc comments
 * [ ] add doc comments to my code
 */

/// Simple structure to represent a task.
///
/// default done = false
/// set done = true when the task is completed
struct TaskItem {
    name: String,
    done: bool,
}

impl TaskItem {
    /// Gives you the `TaskItem` as a yaml represantion
    /// provide a `node_depth`. The depth will insert two spaces per number.
    ///
    /// Example: missing
    fn to_yaml(&self, node_depth: u8) -> String {
        let prefix = String::from("  ");
        let mut prefix_depth = String::new();
        let eol = String::from("\n");
        for _ in 0..node_depth {
            prefix_depth += &prefix;
        }
        let name_prefix = &"name: ";
        let done_prefix = &"done: ";
        let mut item_as_string = prefix_depth.clone() + name_prefix + &self.name.clone() + &eol;
        item_as_string =
            item_as_string + &prefix_depth + done_prefix + &self.done.to_string() + &eol;
        item_as_string
    }
    /// Gives a `TaskItem` from a yaml string
    /// provide the String
    ///
    /// TODO: Mode with different EOL Symbols?
    /// Do we care about nexted nodes?
    fn from_yaml(yaml: &String) -> Self {
        // parse line by line and match key to struct
        let name_node = String::from("name: ");
        let eol = String::from("\n");

        let mut name_str = yaml.trim_start();
        let mut pos = match name_str.find(&name_node) {
            Some(x) => x,
            None => panic!(),
        };

        let mut start_pos = pos + name_node.len();
        let mut end_pos = match name_str.find(&eol) {
            Some(x) => x,
            None => panic!(),
        };
        let extract_name = name_str[start_pos..end_pos].to_string();

        // new pos is end position of extractued value plus the end of line lenght
        // eol can differ from os
        start_pos = end_pos + eol.len();
        name_str = name_str[start_pos..].trim_start();

        let done_node = String::from("done: ");
        pos = match name_str.find(&done_node) {
            Some(x) => x,
            None => panic!(),
        };

        start_pos = pos + done_node.len();
        end_pos = match name_str.find(&eol) {
            Some(x) => x,
            None => panic!(),
        };
        let extract_done = match &name_str[start_pos..end_pos] {
            "true" => true,
            "false" => false,
            _ => panic!(),
        };

        TaskItem {
            name: extract_name,
            done: extract_done,
        }
    }
}

struct SortedTaskList {
    data: BTreeMap<usize, TaskItem>,
}

impl SortedTaskList {
    fn new() -> Self {
        SortedTaskList {
            data: BTreeMap::new(),
        }
    }

    fn remove_by_index(&mut self, index: usize) {
        // I could use the rutrn value Some, None to print a verbose output
        // Could not remove because it's not in the list?
        self.data.remove(&index);
    }

    fn push(&mut self, task: TaskItem) {
        let index = self.data.len();
        self.data.insert(index, task);
    }

    fn task_done_by_index(&mut self, index: usize) {
        if let Some(task_item) = self.data.get(&index) {
            let updated_task = TaskItem {
                name: task_item.name.clone(),
                done: true,
            };
            self.data.insert(index, updated_task);
        } else {
            println!("Error: cannot find task with index {}", index);
        }
    }

    fn print(&self) {
        for (_, item) in self.data.iter() {
            println!(
                "{} - {}",
                match item.done {
                    true => "[X]",
                    false => "[ ]",
                },
                item.name
            );
        }
    }

    /// TaskList as yaml string
    fn to_yaml(&self) -> String {
        let root_node_name = String::from("task:\n");
        let mut yaml = String::new();
        for (_, item) in &self.data {
            let node_yaml = root_node_name.clone() + &item.to_yaml(1);
            yaml += &node_yaml;
        }
        yaml
    }

    /// Build TaskList from yaml string
    fn from_yaml(yaml: String) -> Self {
        let root_node_name = String::from("task:\n");

        let mut root_node_positions = Vec::new();
        let mut start_pos = 0;

        while let Some(pos) = yaml[start_pos..].find(&root_node_name) {
            let absolute_pos = start_pos + pos;
            root_node_positions.push(absolute_pos);
            start_pos = absolute_pos + root_node_name.len();
            root_node_positions.push(start_pos); // i want to extract the string behind last
        }

        let mut tasks = SortedTaskList {
            data: BTreeMap::new(),
        };

        // I always skip the 1st Element, because i will go from behind root node to begin
        // root node and i store always begin and end
        //
        // but my logic false short also for the last element
        // becaus i will only have a end an my if (index +1) will trigger
        let mut toggle = true;
        for (index, &i) in root_node_positions.iter().enumerate() {
            // is "if x % 2 faster then "if aBool" and "toggle bool" - I always skip the 0, 2, 4...
            // atm i think: YES! Modulo is super performant in every chipset and one operation
            // instead of two is much faster here. I mean storing toggle is even a slow memory
            // operation, right? Maybe the rust compiler will just optimize this code, so it
            // doenst matter. I need to look at the assembly and find out myself
            if toggle {
                toggle = false;
                continue;
            }
            let from_pos;
            let to_pos;
            if (index + 1) == root_node_positions.len() {
                // last element
                from_pos = i;
                to_pos = yaml.len();
            } else {
                from_pos = i;
                to_pos = root_node_positions[index + 1];
            }
            let a_item = TaskItem::from_yaml(&yaml[from_pos..to_pos].to_string());
            tasks.push(a_item);
            toggle = true;
        }
        tasks
    }
}

struct TaskFileIO {
    path: String,
    filename: String,
}

/// Read and write `SortedTaskList` as file.
impl TaskFileIO {
    /// always in the current `.` directory
    /// only works with filename as `default.task`
    fn new() -> Self {
        TaskFileIO {
            path: String::from("./"), // change this for debug only later but i dont know
            filename: String::from("default.task"),
        }
    }

    /// Will write the yaml string to the specified file
    /// Set `path` and `fileneame` before usage 
    fn write_file(&self, yaml: String) -> () {
        let full_path = format!("{}{}", self.path, self.filename);
        let path = std::path::Path::new(&full_path);
        let display = path.display();

        // change write mode later on
        // I dont wanna desotroy the content in file or do I?
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&full_path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(yaml.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
    
    /// reads from file and 
    /// returns the yamls as std::String
    fn read_file(&self) -> String {
        let full_path = format!("{}{}", self.path, self.filename);
        let path = std::path::Path::new(&full_path);
        let display = path.display();
        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let mut yaml_as_string = String::new();
        match file.read_to_string(&mut yaml_as_string) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} read\n", display),
        }
        yaml_as_string
    }
}

enum Action {
    Add,
    List,
    Done,
    Nop, // No Operation
}

struct TaskOperator {
    list: SortedTaskList,
}

impl TaskOperator {
    fn new(yaml: String) -> Self {
        TaskOperator {
            list: SortedTaskList::from_yaml(yaml),
        }
    }

    fn parse_commandstring(&mut self, args: Vec<String>) {
        // simple state machine with two states { false, true }
        // because we take one argmuent decide what is it is, set a state and do stuff in that

        // i have two options to process further own, just execute the corresponding function when
        // i have the cli input but then i cannot interact between two actions or do stuff in
        // between or before executing that things
        //
        // i will just start with a stright execute for my alpha version
        // and refactor that later on into a action list which will get returned and give more
        // possibilities and better code to maintain in the long run
        // also i dont need to pass task_list into this function which doesnt make sense

        let mut parse_option = Action::Nop;

        for parameter in args.iter().skip(1) {
            match parse_option {
                Action::Nop => {
                    // default: when we dont have a multi value input or just a new one
                    match parameter.as_str() {
                        "add" => {
                            parse_option = Action::Add;
                        }
                        "list" => {
                            self.list.print();
                            parse_option = Action::Nop;
                        }
                        "done" => {
                            parse_option = Action::Done;
                        }
                        _ => {
                            println!("unkown option");
                            // just abort with message how to use the programm?
                        }
                    }
                }
                Action::Add => {
                    println!("added value {}", parameter);
                    let new_task = TaskItem {
                        name: parameter.clone(),
                        done: false,
                    };
                    self.list.push(new_task);
                }
                Action::Done => {
                    let index = match parameter.parse::<usize>() {
                        Ok(u) => u,
                        Err(e) => panic!(), // print error message abort
                    };
                    self.list.task_done_by_index(index);
                }
                _ => {
                    println!("no action");
                    // panic with error? bug?
                    // we should never reach this state!
                }
            }
        }
    }
}

fn main() {
    /* create io
     * get command line options and argmunets
     */
    let task_file_io = TaskFileIO::new();
    let args: Vec<String> = env::args().collect();

    /* at startup
     * read everything from default file
     */
    let mut cli = TaskOperator::new(task_file_io.read_file());
    cli.parse_commandstring(args);

    /* before end 
     * save everything
     */
    task_file_io.write_file(cli.list.to_yaml());
}
