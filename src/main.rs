use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use std::fs::File;
use std::io::prelude::*;

/* https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
 * https://github.com/dtolnay/serde-yaml
 *
 *
 * TODO
 * [X] refactore read write into file type / class / interfaces whatever?
 * [X] Change Yaml format, Index Information is already stored by order and not needed!
 * [ ] -> serde::yaml doesnt fit my requirements
 *      -> I need to write my own parser
 *      -> I have a super limited scope so its actually finde to do
 *      [X] write to_yaml for TaskItem
 *      [ ] read from_yaml for TaskItem
 *      [ ] refactor fn *yaml for SortedTaskList
 *
 * TODO next week
 * [ ] build full POC with
 *      - add task
 *      - delete task
 *      - list tasks
 *
 * TODO week after
 * [ ] build with nix
 * [ ] add cli support
 *      - move up
 *      - move down
 */

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TaskItem {
    name: String,
    priority: u8,
    done: bool,
}

impl TaskItem {
    fn to_yaml(&self, node_depth: u8) -> String {
        let prefix = String::from("  ");
        let mut prefix_depth = String::new();
        let eol = String::from("\n");
        for _ in 0 .. node_depth {
            prefix_depth += &prefix;
        }
        let name_prefix = &"name: ";
        let priority_prefix = &"prio: ";
        let done_prefix = &"done: ";
        let mut item_as_string = prefix_depth.clone() + name_prefix + &self.name.clone() + &eol;
        item_as_string = item_as_string + &prefix_depth + priority_prefix + &self.priority.to_string() + &eol;
        item_as_string = item_as_string + &prefix_depth + done_prefix + &self.done.to_string() + &eol;
        item_as_string
    }
    fn from_yaml(&self) -> String {
        todo!()
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
        self.data.remove(&index);
    }

    fn push(&mut self, task_name: String) {
        // default behavior is prio 50 and done = false
        let new_task = TaskItem {
            name: task_name.clone(),
            priority: 50,
            done: false,
        };
        let index = self.data.len();
        self.data.insert(index, new_task);
    }

    fn print(&self) {
        for (_, item) in self.data.iter() {
            println!("Task: {}, Done = {}", item.name, item.done);
        }
    }



    fn to_yaml(&self) -> String {
        // let yaml = serde_yaml::to_string(&self.data);
        // match yaml {
        //     Ok(yaml) => yaml,
        //     Err(_error) => panic!("empty"),
        // }
        let root_node_name = String::from("task:\n");
        let mut yaml = String::new();
        for (_, item) in &self.data {
            let node_yaml = root_node_name.clone() + &item.to_yaml(1);
            yaml += &node_yaml;
        }
        yaml
    }

    fn from_yaml(tasks: &str) -> Self {
        SortedTaskList {
            data: serde_yaml::from_str(tasks).unwrap(),
        }
    }
}

struct TaskFileIO {
    path: String,
    filename: String,
}

impl TaskFileIO {
    fn new() -> Self {
        TaskFileIO {
            path: String::from("./"), // change this for debug only later but i dont know
            filename: String::from("default.task"),
        }
    }

    fn write_file(&self, yaml: String) -> () {
        // we only operate in the current directory
        // let mut path = std::path::Path::new(&self.path);
        // let display = path.display();
        // if !path.exists() {
        //     let res = std::fs::create_dir(path);
        //     match res {
        //         Ok(x) => (),
        //         Err(e) => panic!("cannot create directory!"),
        //     }
        // }

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

fn main() {
    let mut task_list = SortedTaskList::new();
    for i in 1..10 {
        task_list.push("task ".to_string() + &i.to_string());
    }
    task_list.remove_by_index(0);
    task_list.remove_by_index(1);
    //task_list.print();

    println!("----------------------");

    let task_file_io = TaskFileIO::new();
    task_file_io.write_file(task_list.to_yaml());
    let task_list_from_file = SortedTaskList::from_yaml(&task_file_io.read_file());
    task_list_from_file.print();
}
