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

fn main() {
    let mut current_action: Action;


    let action_a = Action::Add;
    let action_d = Action::Delete;
    let action_e = Action::Edit;

    current_action = action_a;
    inspect_action(current_action);
    current_action = action_d;
    inspect_action(current_action);
    current_action = action_e;
    inspect_action(current_action);
}
