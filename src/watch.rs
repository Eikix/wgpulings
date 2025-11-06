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
        println!(
            "{} Current exercise: {}\n",
            "→".cyan().bold(),
            exercise.name
        );
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
                        let passed = if let Ok(()) = exercise.compile() {
                            // For Run mode, also check that execution succeeds
                            if exercise.mode == crate::exercise::Mode::Run {
                                if let Ok(output) = exercise.run() {
                                    // Validate success
                                    let has_success = output.contains("🎉")
                                        || output.contains("Success!")
                                        || (output.contains("✓") && (
                                            output.contains("correct")
                                            || output.contains("Perfect")
                                            || output.contains("All passes executed correctly")
                                        ));

                                    let has_failure = output.contains("❌")
                                        || output.contains("don't match")
                                        || output.contains("incorrect")
                                        || output.contains("wrong")
                                        || output.contains("failed");

                                    has_success && !has_failure
                                } else {
                                    false
                                }
                            } else {
                                true
                            }
                        } else {
                            false
                        };

                        if passed {
                            println!(
                                "\n{} Exercise completed! Moving to next...\n",
                                "✓".green().bold()
                            );

                            // Find next incomplete exercise
                            let current_idx = exercises
                                .iter()
                                .position(|e| e.name == exercise.name)
                                .unwrap();
                            current_exercise = exercises
                                .iter()
                                .skip(current_idx + 1)
                                .find(|e| !e.is_done());

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
        println!(
            "\n{} Exercise contains \"I AM NOT DONE\"",
            "⚠".yellow().bold()
        );
        println!("Remove this marker when you're ready to check your solution.");
        println!("\n{} Hint: wgpulings hint {}", "💡".yellow(), exercise.name);
        return;
    }

    print!("Compiling... ");
    match exercise.compile() {
        Ok(_) => {
            println!("{}", "✓".green().bold());

            // If it's a Run mode exercise, execute it and validate output
            if exercise.mode == crate::exercise::Mode::Run {
                println!("\nExecuting...\n");
                match exercise.run() {
                    Ok(output) => {
                        // Print the output
                        if !output.is_empty() {
                            println!("{}", output);
                        }

                        // Validate that the exercise succeeded
                        // Success indicators: 🎉, ✓ followed by success messages
                        // Failure indicators: ❌, "don't match", "incorrect", "wrong"
                        let has_success = output.contains("🎉")
                            || output.contains("Success!")
                            || (output.contains("✓") && (
                                output.contains("correct")
                                || output.contains("Perfect")
                                || output.contains("All passes executed correctly")
                            ));

                        let has_failure = output.contains("❌")
                            || output.contains("don't match")
                            || output.contains("incorrect")
                            || output.contains("wrong")
                            || output.contains("failed");

                        if has_success && !has_failure {
                            println!("\n{} Exercise completed successfully!", "✓".green().bold());
                            println!("Ready to move to the next one!");
                        } else if has_failure {
                            println!("\n{} Exercise ran but results don't match expected values!", "⚠".yellow().bold());
                            println!("Keep working on it!");
                        } else {
                            // No clear success or failure indicators
                            println!("\n{} Exercise executed!", "✓".green().bold());
                            println!("Ready to move to the next one!");
                        }
                    }
                    Err(e) => {
                        println!("\n{} Runtime error:", "✗".red().bold());

                        // Show first few lines of error
                        for line in e.lines().take(15) {
                            println!("{}", line);
                        }

                        println!("\n{} Hint: wgpulings hint {}", "💡".yellow(), exercise.name);
                    }
                }
            } else {
                println!("\n{} Exercise compiled successfully!", "✓".green().bold());
                println!("Ready to move to the next one!");
            }
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
