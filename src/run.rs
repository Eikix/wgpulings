use crate::exercise::Exercise;
use colored::*;

pub fn run(exercise: &Exercise) {
    println!("\n{} {}", "Running".cyan().bold(), exercise.name);
    println!("{}", "=".repeat(50));

    if !exercise.is_done() {
        println!(
            "\n{} This exercise still contains \"I AM NOT DONE\"",
            "⚠".yellow().bold()
        );
        println!("Remove this marker when you think you've solved the exercise.\n");
        return;
    }

    print!("Compiling... ");
    match exercise.compile() {
        Ok(_) => {
            println!("{}", "✓".green().bold());

            if exercise.mode == crate::exercise::Mode::Run {
                println!("\nExecuting...\n");
                match exercise.run() {
                    Ok(output) => {
                        if !output.is_empty() {
                            println!("{}", output);
                        }
                        println!("\n{} Exercise completed successfully!", "✓".green().bold());
                    }
                    Err(e) => {
                        println!("\n{} Runtime error:", "✗".red().bold());
                        println!("{}", e);
                    }
                }
            } else {
                println!("\n{} Exercise compiled successfully!", "✓".green().bold());
            }
        }
        Err(e) => {
            println!("{}", "✗".red().bold());
            println!("\n{} Compilation failed:", "✗".red().bold());
            println!("{}", e);
            println!("\n{} Need help? Try: wgpulings hint {}", "💡".yellow(), exercise.name);
        }
    }
}
