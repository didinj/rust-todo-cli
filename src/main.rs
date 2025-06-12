mod task;
use task::{ Task, load_tasks, save_tasks };

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut tasks = load_tasks();

    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            let title = args.get(2).expect("No task title provided");
            tasks.push(Task {
                title: title.clone(),
                done: false,
            });
            save_tasks(&tasks);
            println!("Task added.");
        }
        Some("list") => {
            for (i, task) in tasks.iter().enumerate() {
                let status = if task.done { "✓" } else { " " };
                println!("[{}] {}: {}", status, i, task.title);
            }
        }
        Some("done") => {
            let index: usize = args.get(2).unwrap().parse().unwrap();
            if let Some(task) = tasks.get_mut(index) {
                task.done = true;
                save_tasks(&tasks);
                println!("Task marked as done.");
            }
        }
        _ => {
            println!("Usage: todo_cli [add|list|done] [task]");
        }
    }
}
