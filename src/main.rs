use std::io::{self, Write};

struct Task {
    title: String,
    completed: bool,
}

fn main() -> io::Result<()> {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n=======================");
        println!("1. Add a task");
        println!("2. List tasks");
        println!("3. Mark a task as complete");
        println!("4. Exit");

        let choice = prompt("Enter your choice: ")?;

        match choice.as_str() {
            "1" => add_task(&mut tasks)?,
            "2" => list_tasks(&tasks),
            "3" => mark_task_complete(&mut tasks)?,
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option. Enter a number from 1 to 4."),
        }
    }

    Ok(())
}

fn prompt(message: &str) -> io::Result<String> {
    print!("{message}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())
}

fn add_task(tasks: &mut Vec<Task>) -> io::Result<()> {
    let title = prompt("Enter task title: ")?;

    if title.is_empty() {
        println!("Task title cannot be empty.");
        return Ok(());
    }

    tasks.push(Task {
        title,
        completed: false,
    });

    println!("Task added.");
    Ok(())
}

fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks yet.");
        return;
    }

    println!("\nTASKS\n-----");
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "x" } else { " " };
        println!("{}. [{}] {}", index + 1, status, task.title);
    }
}

fn mark_task_complete(tasks: &mut [Task]) -> io::Result<()> {
    if tasks.is_empty() {
        println!("No tasks to complete.");
        return Ok(());
    }

    list_tasks(tasks);
    let input = prompt("Enter task number to complete: ")?;
    let number = match input.parse::<usize>() {
        Ok(number) => number,
        Err(_) => {
            println!("Please enter a valid task number.");
            return Ok(());
        }
    };

    if number == 0 || number > tasks.len() {
        println!("No task exists with that number.");
        return Ok(());
    }

    let task = &mut tasks[number - 1];
    if task.completed {
        println!("That task is already complete.");
    } else {
        task.completed = true;
        println!("Task marked as complete.");
    }

    Ok(())
}
