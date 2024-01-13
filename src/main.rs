use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new C++ project
    New {
        /// Sets the name of the project
        #[clap(short, long)]
        name: String,

        /// Sets the source directory
        #[clap(short, long, default_value = "src")]
        src_dir: String,

        /// Sets the include directory
        #[clap(short, long, default_value = "include")]
        include_dir: String,

        /// Sets the build directory
        #[clap(short, long, default_value = "build")]
        build_dir: String,

        /// Sets the executable directory
        #[clap(short, long, default_value = "bin")]
        exec_dir: String,
    },
    /// Initializes and runs set up for the C++ project
    Init {
        /// Sets the source directory
        #[clap(short, long, default_value = "src")]
        src_dir: String,

        /// Sets the build directory
        #[clap(short, long, default_value = "build")]
        build_dir: String,
    },
    /// Builds the C++ project
    Build {
        // /// Sets the source directory
        // #[clap(short, long, default_value = "src")]
        // src_dir: String,

        // /// Sets the include directory
        // #[clap(short, long, default_value = "include")]
        // include_dir: String,
        /// Sets the build directory
        #[clap(short, long, default_value = "build")]
        build_dir: String,
        // /// Sets the executable directory
        // #[clap(short, long, default_value = "bin")]
        // runtime_dir: String,

        // /// Sets the executable name
        // #[clap(short, long, default_value = None)]
        // exec_name: Option<String>,
    },
    /// Runs the built C++ project
    Run {
        // /// Specifies the source directory
        // #[clap(short, long, default_value = "src")]
        // src_dir: String,

        // /// Specifies the include directory
        // #[clap(short, long, default_value = "include")]
        // include_dir: String,

        /// Specifies the build directory
        #[clap(short, long, default_value = "build")]
        build_dir: String,

        /// Specifies the executable directory
        #[clap(short, long, default_value = "bin")]
        runtime_dir: String,

        /// Specifies the executable name
        #[clap(short, long, default_value = None)]
        exec_name: Option<String>,

        /// Specifies the executable arguments
        #[clap(short, long, default_value = None)]
        args: Option<Vec<String>>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::New {
            name,
            src_dir,
            include_dir,
            build_dir,
            exec_dir,
        } => {
            if std::path::Path::new(&name).exists() {
                println!("Error: Project already exists");
                return;
            }

            println!("\nCreating new project: {}\n", name);

            let src_dir = format!("{}/{}", name, src_dir);
            let include_dir = format!("{}/{}", name, include_dir);
            let build_dir = format!("{}/{}", name, build_dir);
            let exec_dir = format!("{}/{}", name, exec_dir);

            std::fs::create_dir_all(&src_dir).unwrap();
            std::fs::create_dir_all(&include_dir).unwrap();
            std::fs::create_dir_all(&build_dir).unwrap();
            std::fs::create_dir_all(&exec_dir).unwrap();

            std::fs::write(
                format!("./{}/.gitignore", &name),
                format!(
                    "
# C++
*.o
*.out
*.exe
*.dll
*.so
*.dylib
"
                ),
            )
            .unwrap();

            std::fs::write(
                format!("./{}/CMakeLists.txt", &name),
                format!(
                    "
cmake_minimum_required(VERSION 3.24)
project({name})

# Set compiler flags
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_FLAGS \"${{CMAKE_CXX_FLAGS}} -Wall -Werror -Wextra -pedantic -pedantic-errors -g\")

# Include project headers
include_directories(./{include_dir})
# Define the source files and dependencies for the executable
set(SOURCE_FILES {src_dir}/main.cpp)

# Make the project root directory the working directory when we run
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${{CMAKE_CURRENT_SOURCE_DIR}}/{exec_dir})
set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)
add_executable({name} ${{SOURCE_FILES}})
"
                ),
            )
            .unwrap();

            std::fs::write(
                format!("./{}/main.cpp", &src_dir),
                format!(
                    "
#include <iostream>

int main() {{
  std::cout << \"Hello, world!\" << std::endl;
  return 0;
}}
"
                ),
            )
            .unwrap();

            Command::new("zsh")
                .arg("-c")
                .arg(format!("cmake -S {} -B {}", src_dir, build_dir))
                .output()
                .expect("Error: Failed to open project directory");

            Command::new("zsh")
                .arg("-c")
                .arg(format!("git init"))
                .output()
                .expect("Error: Failed to run git init");

            Command::new("zsh")
                .arg("-c")
                .arg(format!("git add ."))
                .output()
                .expect("Error: Failed to run git add");

            Command::new("zsh")
                .arg("-c")
                .arg(format!("git commit -m \"Initial commit\""))
                .output()
                .expect("Error: Failed to run git commit");
        }
        Commands::Init { src_dir, build_dir } => {
            println!("\nInitializing project...\n");

            Command::new("zsh")
                .arg("-c")
                .arg(format!("cmake -S {} -B {}", src_dir, build_dir))
                .output()
                .expect("Error: Failed to run cmake");
        }
        Commands::Build {
            // src_dir: _,
            // include_dir: _,
            build_dir,
            // runtime_dir: _,
            // exec_name: _,
        } => {
            println!("\nBuilding project...\n");

            Command::new("zsh")
                .arg("-c")
                .arg(format!("cmake --build {}", build_dir))
                .output()
                .expect("Error: Failed to run cmake");
        }
        Commands::Run {
            // src_dir: _,
            // include_dir: _,
            build_dir,
            runtime_dir,
            exec_name,
            args,
        } => {
            println!("\nBuilding project...\n");

            Command::new("zsh")
                .arg("-c")
                .arg(format!("cmake --build {}", build_dir))
                .output()
                .expect("Error: Failed to run cmake");

            println!("\nRunning project...\n");

            Command::new("zsh")
                .arg("-c")
                .arg(format!(
                    "cd {exec_dir} && ./{exec_name} {exec_args} && cd ..",
                    exec_dir = runtime_dir,
                    exec_name = match exec_name {
                        Some(name) => name,
                        None => Command::new("zsh")
                            .arg("-c")
                            .arg(format!("basename $(pwd)"))
                            .output()
                            .expect("Error: Failed to run basename")
                            .stdout
                            .into_iter()
                            .map(|c| c as char)
                            .collect::<String>(),
                    },
                    exec_args = args
                        .and_then(|args| Some(args.join(" ")))
                        .unwrap_or("".to_string())
                ))
                .output()
                .expect("Error: Failed to run executable");
        }
    }
}
