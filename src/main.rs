use clap::{Parser, Subcommand};
use colored::*;

mod exercise;
mod run;
mod verify;
mod watch;

use exercise::Exercise;

#[derive(Parser)]
#[command(name = "wgpulings")]
#[command(about = "Small exercises to get you used to reading and writing WGSL shaders and wgpu code", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify all exercises
    Verify,

    /// Watch exercises and run them on changes
    Watch,

    /// Run a specific exercise
    Run {
        /// Name of the exercise to run
        name: String,
    },

    /// Get a hint for an exercise
    Hint {
        /// Name of the exercise
        name: String,
    },

    /// List all exercises
    List {
        /// Show only incomplete exercises
        #[arg(short, long)]
        unsolved: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let exercises = Exercise::load_all().unwrap_or_else(|e| {
        eprintln!("{} Failed to load exercises: {}", "ERROR".red().bold(), e);
        std::process::exit(1);
    });

    match cli.command {
        Commands::Verify => {
            verify::verify(&exercises);
        }
        Commands::Watch => {
            watch::watch(&exercises);
        }
        Commands::Run { name } => {
            let exercise = exercises
                .iter()
                .find(|e| e.name == name)
                .unwrap_or_else(|| {
                    eprintln!("{} Exercise '{}' not found", "ERROR".red().bold(), name);
                    std::process::exit(1);
                });

            run::run(exercise);
        }
        Commands::Hint { name } => {
            let exercise = exercises
                .iter()
                .find(|e| e.name == name)
                .unwrap_or_else(|| {
                    eprintln!("{} Exercise '{}' not found", "ERROR".red().bold(), name);
                    std::process::exit(1);
                });

            println!("\n{}", "Hint:".yellow().bold());
            println!("{}", exercise.hint);
        }
        Commands::List { unsolved } => {
            list_exercises(&exercises, unsolved);
        }
    }
}

fn list_exercises(exercises: &[Exercise], unsolved_only: bool) {
    println!("\n{}", "Exercises:".bold());
    println!("{}", "==========".bold());

    let mut current_section = "";

    for (i, exercise) in exercises.iter().enumerate() {
        let is_done = exercise.is_done();

        if unsolved_only && is_done {
            continue;
        }

        // Print section header if changed
        let section = exercise.path.split('/').nth(1).unwrap_or("");
        if section != current_section {
            current_section = section;
            println!("\n{}", section.to_uppercase().cyan().bold());
        }

        let status = if is_done { "✓".green() } else { "○".red() };

        println!(
            "  {} {:<3} {:<15} {}",
            status,
            i + 1,
            exercise.name,
            exercise.path
        );
    }

    let total = exercises.len();
    let completed = exercises.iter().filter(|e| e.is_done()).count();

    println!("\n{}", "Progress:".bold());
    println!("  {}/{} exercises completed", completed, total);

    if completed == total {
        println!(
            "\n{}",
            "🎉 Congratulations! You've completed all exercises!"
                .green()
                .bold()
        );
    }
}
