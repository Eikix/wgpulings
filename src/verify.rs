use crate::exercise::Exercise;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

pub fn verify(exercises: &[Exercise]) {
    println!("\n{}", "Verifying all exercises...".bold());
    println!("{}", "=".repeat(50));

    let progress = ProgressBar::new(exercises.len() as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .expect("Invalid progress style")
            .progress_chars("=> "),
    );

    let mut failed = Vec::new();

    for exercise in exercises {
        progress.set_message(format!("Checking {}", exercise.name));

        if !exercise.is_done() {
            failed.push((
                exercise,
                "Not completed (contains 'I AM NOT DONE')".to_string(),
            ));
            progress.inc(1);
            continue;
        }

        match exercise.compile() {
            Ok(_) => {
                // Success
            }
            Err(e) => {
                failed.push((exercise, e));
            }
        }

        progress.inc(1);
    }

    progress.finish_and_clear();

    println!("\n{}", "Results:".bold());
    println!("{}", "=".repeat(50));

    if failed.is_empty() {
        println!("\n{} All exercises passed!", "✓".green().bold());
        println!("\n🎉 {} 🎉", "Congratulations!".green().bold());
        println!("You've completed all the wgpulings exercises!");
        println!("You're now ready to write GPU code with wgpu and WGSL!");
    } else {
        println!(
            "\n{} {} exercise(s) failed:\n",
            "✗".red().bold(),
            failed.len()
        );

        for (exercise, error) in &failed {
            println!("  {} {}", "✗".red(), exercise.name);
            if !error.contains("I AM NOT DONE") {
                println!("    {}", error.lines().next().unwrap_or("Unknown error"));
            }
        }

        println!(
            "\n{} Get hints with: wgpulings hint <exercise_name>",
            "💡".yellow()
        );
        std::process::exit(1);
    }
}
