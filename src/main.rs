use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/* https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
 * https://github.com/dtolnay/serde-yaml
 *
 *
 * TODO today
 * [X] write yaml to file
 * [X] read yaml from file
 * [X] transfer data from read buffer into a my data format
 *
 * TODO next week
 * [ ] build full POC with
 *      - add task
 *      - delete task
 *      - list tasks
 *      - move up
 *      - move down
 */

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TaskItem {
    name: String,
    priority: u8,
    done: bool,
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
        let yaml = serde_yaml::to_string(&self.data);
        match yaml {
            Ok(yaml) => yaml,
            Err(_error) => panic!("empty"),
        }
    }

    fn from_yaml(tasks: &str) -> Self {
        SortedTaskList {
            data: serde_yaml::from_str(tasks).unwrap(),
        }
    }

    fn write_file(&self) -> std::io::Result<()> {
        let filename = String::from("testoutput/task.yaml");
        // default is task.yaml; maybe something like default.task later wich will
        // get read automatically on startup?
        let result = std::fs::write(filename, self.to_yaml());
        result
    }

    fn read_file() -> Self {
        let input = std::fs::read_to_string("testoutput/task.yaml");
        let yaml = match input {
            Ok(yaml) => yaml,
            Err(_error) => panic!("empty"),
        };
        SortedTaskList::from_yaml(&yaml)
    }
}

fn main() {
    let mut task_list = SortedTaskList::new();
    for i in 1..10 {
        task_list.push("task ".to_string() + &i.to_string());
    }
    task_list.remove_by_index(0);
    task_list.remove_by_index(1);
    task_list.print();

    let _ = task_list.write_file();

    println!("----------------------");
    let new_list = SortedTaskList::read_file();
    new_list.print();
}
