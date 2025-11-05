use crate::exercise::Exercise;
use colored::*;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch(exercises: &[Exercise]) {
    println!("\n{}", "Watch mode activated!".green().bold());
    println!("{}", "=".repeat(50));
    println!("Watching for file changes in exercises/...");
    println!("Press Ctrl+C to exit\n");

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default(),
    )
    .expect("Failed to create watcher");

    watcher
        .watch("exercises".as_ref(), RecursiveMode::Recursive)
        .expect("Failed to watch exercises directory");

    // Find first incomplete exercise
    let mut current_exercise = exercises.iter().find(|e| !e.is_done());

    if let Some(exercise) = current_exercise {
        println!("{} Current exercise: {}\n", "→".cyan().bold(), exercise.name);
        check_exercise(exercise);
    } else {
        println!("{} All exercises completed!", "✓".green().bold());
        return;
    }

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(_event) => {
                // Debounce: wait a bit for multiple events
                std::thread::sleep(Duration::from_millis(100));

                // Clear any pending events
                while rx.try_recv().is_ok() {}

                // Re-check current exercise
                if let Some(exercise) = current_exercise {
                    println!("\n{}", "File changed, rechecking...".yellow());
                    check_exercise(exercise);

                    // If completed, move to next
                    if exercise.is_done() {
                        if let Ok(()) = exercise.compile() {
                            println!("\n{} Exercise completed! Moving to next...\n", "✓".green().bold());

                            // Find next incomplete exercise
                            let current_idx = exercises.iter().position(|e| e.name == exercise.name).unwrap();
                            current_exercise = exercises.iter().skip(current_idx + 1).find(|e| !e.is_done());

                            if let Some(next) = current_exercise {
                                println!("{} Next exercise: {}\n", "→".cyan().bold(), next.name);
                                check_exercise(next);
                            } else {
                                println!("\n🎉 {} 🎉", "All exercises completed!".green().bold());
                                break;
                            }
                        }
                    }
                }
            }
            Err(_) => {
                // Timeout, just continue
            }
        }
    }
}

fn check_exercise(exercise: &Exercise) {
    println!("{}", "=".repeat(50));
    println!("{} {}", "Checking".cyan().bold(), exercise.name);
    println!("{}", "=".repeat(50));

    if !exercise.is_done() {
        println!("\n{} Exercise contains \"I AM NOT DONE\"", "⚠".yellow().bold());
        println!("Remove this marker when you're ready to check your solution.");
        println!("\n{} Hint: wgpulings hint {}", "💡".yellow(), exercise.name);
        return;
    }

    print!("Compiling... ");
    match exercise.compile() {
        Ok(_) => {
            println!("{}", "✓".green().bold());
            println!("\n{} Exercise looks good!", "✓".green().bold());
            println!("Ready to move to the next one!");
        }
        Err(e) => {
            println!("{}", "✗".red().bold());
            println!("\n{} Compilation failed:", "✗".red().bold());

            // Show first few lines of error
            for line in e.lines().take(10) {
                println!("{}", line);
            }

            println!("\n{} Hint: wgpulings hint {}", "💡".yellow(), exercise.name);
        }
    }
}
