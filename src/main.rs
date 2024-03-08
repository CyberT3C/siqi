// An attribute to hide warnings for unused code.
// we are learning atm, so lets just try things
#![allow(dead_code)]

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

    use crate::Action::*;
    current_action = Add;
    inspect_action(current_action);
    current_action = Delete;
    inspect_action(current_action);
    current_action = Edit;
    inspect_action(current_action);

}
