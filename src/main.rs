use std::io;
use std::io::Write;



fn main() {
		let mut tasks: Vec<Task> = Vec::new();

    println!("\n1. Add a task\n2. List tasks\n3. Mark a task as complete\n4. Exit\n");
    print!("Enter your choice : ");
    io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("You entered: {}", input.trim());
}

struct Task {
	title: String,
	completed: bool
}