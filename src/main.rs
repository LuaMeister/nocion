
struct Action {
    name: &'static str,
    run: fn(),
}

const ACTIONS: [Action; 1] = [
    Action {
        name: "generate",
        run: || println!("Hello, world!")
    }
];

fn main() {
    let action_name = std::env::args().nth(1);
    
    if let Some(action_name) = action_name {
        for action in ACTIONS.iter() {
            if action.name == action_name {
                (action.run)();
                return;
            }
        }
    }
}