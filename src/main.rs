use std::collections::BTreeMap;

use std::fs::File;
use std::io::prelude::*;

/* https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
 *
 *
 * TODO
 * [X] refactore read write into file type / class / interfaces whatever?
 * [X] Change Yaml format, Index Information is already stored by order and not needed!
 * [X] -> serde::yaml doesnt fit my requirements
 *      -> I need to write my own parser
 *      -> I have a super limited scope so its actually finde to do
 *      [X] write to_yaml for TaskItem
 *      [X] read from_yaml for TaskItem
 *      [X] refactor fn *yaml for SortedTaskList
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

struct TaskItem {
    name: String,
    done: bool,
}

impl TaskItem {
    fn to_yaml(&self, node_depth: u8) -> String {
        let prefix = String::from("  ");
        let mut prefix_depth = String::new();
        let eol = String::from("\n");
        for _ in 0..node_depth {
            prefix_depth += &prefix;
        }
        let name_prefix = &"name: ";
        //let priority_prefix = &"prio: ";
        let done_prefix = &"done: ";
        let mut item_as_string = prefix_depth.clone() + name_prefix + &self.name.clone() + &eol;
        //item_as_string =
        //   item_as_string + &prefix_depth + priority_prefix + &self.priority.to_string() + &eol;
        item_as_string =
            item_as_string + &prefix_depth + done_prefix + &self.done.to_string() + &eol;
        item_as_string
    }
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
            println!("index not found");
        }
        
    }

    fn print(&self) {
        for (_, item) in self.data.iter() {
            println!("Task: {}, Done = {}", item.name, item.done);
        }
    }

    fn to_yaml(&self) -> String {
        let root_node_name = String::from("task:\n");
        let mut yaml = String::new();
        for (_, item) in &self.data {
            let node_yaml = root_node_name.clone() + &item.to_yaml(1);
            yaml += &node_yaml;
        }
        yaml
    }

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
            // lets get the tuple?
            // I mean modulo on index maybe much bette performance
            // but the question is do I always skip the 1, 3, 5... or can it change later on ?
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
    for i in 0..9 {
        let new_task = TaskItem {
            name: "task ".to_string() + &i.to_string(),
            done: false,
        };

        task_list.push(new_task);
    }
    //    task_list.remove_by_index(0);
    //    task_list.remove_by_index(1);
    task_list.print();

    println!("----------------------");

    let task_file_io = TaskFileIO::new();
    task_file_io.write_file(task_list.to_yaml());
    let mut task_list_from_file = SortedTaskList::from_yaml(task_file_io.read_file());
    task_list_from_file.task_done_by_index(5);
    task_list_from_file.task_done_by_index(1);
    task_list_from_file.task_done_by_index(100); // print error?
    task_list_from_file.print();
}
