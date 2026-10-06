use std::io;
use std::io::Write;



fn main() {
		let mut tasks: Vec<Task> = Vec::new();

    println!("\nPress number to choose option\n1. Add a task\n2. List tasks\n3. Mark a task as complete\n4. Exit\n");
    print!("Enter your choice : ");
    io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("You entered: {}", input.trim());

    let choice: u32 = input
    .trim()
    .parse()
    .expect("Please enter a number");

match choice {
    1 => {                             //  ADD TASK SCREEN
    	println!("\nADD TASK SCREEN\n");
    }

    2 => {                             // List-tasks screen/action
    	println!("\nLIST TASKS SCREEN\n");
    }
    
    3 => {                             // Complete-task screen/action
    	println!("\nMARK AS COMPLEAT\n");
    }
    4 => {                             // Exit
    	println!("\nEXIT\n");
    }
    _ => {
        println!("INVALID OPTION");
    }
}
}

struct Task {
	title: String,
	completed: bool
}