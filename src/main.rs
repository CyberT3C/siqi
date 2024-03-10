// An attribute to hide warnings for unused code.
// we are learning atm, so lets just try things
//#![allow(dead_code)]
//use std::collections::HashSet;
//use std::collections::HashMap;
use std::collections::BTreeMap;

// lets try to write our binary tree to a file and read it afterwards
use serde::{Deserialize, Serialize};

enum Action {
    Add,
    Delete,
    Edit,
}

fn inspect_action( action: Action) {
    match action {
        Action::Add => println!("add actiion triggered"),
        Action::Edit => println!("edit action triggered"),
        Action::Delete => println!("delete action triggered"),
    }
}

/* Let's learn memory allocation in Rust:w
 * By default all values are stack allocated
 * Values can be boxed - allocated on the heap
 * Create a Box<T>
 * A Box is a smart pointer to a heap allocated value of type T
 * Dereference boxed values with the * operator
 * let value = Box::new(aType);
 * let unxboxed = *value;
 *
 * Vectors are re-sizable arrays
 * Size is not known at compile time. The Capacity indicated how much memory will be reserved.
 * e.g. Vec<u32> there is also a vec![] macro
 * you can yous .push .pop .iter
 *
 * There are two different string types in Rust.
 * String and &str.
 *
 * String is heap allocated always valid utf-8 and basicall a Vec<u8>
 * &str is a slice that points to a utf-8 sequence. It can be used to view into a String
 * like &[T] is a view into Vec<T>
 * String::new() is a emtpy growable string.
 * String::from("some string") is a heap allocated string.
 *
 * Hashmap store values by key. It is actually called HashMap
 * std::collections::HashMap
 * HashMap::with_capacity(uint) or HashMap::new()
 * Any type that implements the Eq and Hash traits can be a key in HashMap
 *  -> HashSet is a HashMap where we just care about the keys
 *  - It guaranteed to not have duplicates :)
 *  Nice: union, difference intersection, symmetric_difference
 *  They shine when you work with multiple Sets!
 */

/* still in learning progress and not sure about the right data struct
 * i think i need to implement a few different ways and caculate the 
 * actuall compute time with different sizes
    data: Vec<TaskItem>,
    index_map: HashMap<String, usize>,
    This is just not efficient!
 *
 * I think my best choice is a binary tree. It will have a good balance between search and cache
 * efficiency
 *
*/
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TaskItem {
    name: String,
    priority: u8,
    done: bool,
}
// usize is for mem indices and sizes
// it depends on the target architeture e.g. 32 or 64 bits
struct SortedTaskList {
//    data: Vec<TaskItem>,
//    index_map: HashMap<String, usize>,
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
       // missing indexing stuff
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
        // this is type result
        let test = match yaml {
            Ok(yaml) => yaml,
            Err(_error) => panic!("empty"),
        };
        test
    }
}
fn main() {
    let mut current_action: Action;

//    let action_a = Action::Add;
//    let action_d = Action::Delete;
//    let action_e = Action::Edit;
//
//    current_action = action_a;
//    inspect_action(current_action);
//    current_action = action_d;
//    inspect_action(current_action);
//    current_action = action_e;
//    inspect_action(current_action);
    
    if true {
        println!("true");
    } else {
        println!("false");
    }

    use crate::Action::*;
    current_action = Add;
    inspect_action(current_action);
    current_action = Delete;
    inspect_action(current_action);
    current_action = Edit;
    inspect_action(current_action);


//    let mut test_hash_map: HashSet<String> = HashSet::new();
//
//    test_hash_map.insert("my first todo".to_string());
//    test_hash_map.insert("lets build functions around the HS".to_string());
//    test_hash_map.insert("Use them to build a module or class maybe crate is the right term in rust".to_string());
//    
//    // iterate
//    for item in &test_hash_map {
//        println!("{item}");
//    }
//    // ok a HashSet is not sorted
//    // so it does not fit for my purpose but good to know
//    for x in test_hash_map.drain() {
//        println!("{x}");
//    }

    // lets lean basic io
    // How do i get some input?
//    println!("Input Task:");
//    let mut user_input = String::new();
//    std::io::stdin()
//        .read_line(&mut user_input)
//        .expect("cannot read user input");
    
    // Interesting just the ';' will stop the line
    // the formatting doesn't matter here so we can write <nice> code :)

//    println!("Input = {user_input}");
//
//    // lets actually fill our list and create a push and pop function ?
//    let mut task_list = SortedTaskList::new();
//    task_list.push(user_input);
//    task_list.push("task 2".to_string());
//    task_list.push("another todo 3".to_string());
//
//    task_list.print();
//    task_list.pop("this is a test".to_string());
    
    let mut task_list = SortedTaskList::new();
    for i in 1 .. 10 { 
        task_list.push("task ".to_string() + &i.to_string());
    }
    task_list.remove_by_index(0);
    task_list.remove_by_index(1);
    task_list.print();

    println!("{}", task_list.to_yaml());
}
