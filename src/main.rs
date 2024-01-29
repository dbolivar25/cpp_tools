use clap::{Parser, Subcommand};
use std::{fmt::Display, process::Command};

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

        /// Sets the file extension for the project
        #[clap(short, long, default_value = "cpp")]
        file_ext: String,

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
        #[clap(short, long, default_value = ".")]
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

enum FileExtensions {
    Cpp,
    C,
}

impl Display for FileExtensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileExtensions::Cpp => write!(f, "cpp"),
            FileExtensions::C => write!(f, "c"),
        }
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::New {
            name,
            file_ext,
            src_dir,
            include_dir,
            build_dir,
            exec_dir,
        } => {
            if std::path::Path::new(&name).exists() {
                println!("Error: Project already exists");
                return;
            }

            let extension = match file_ext.to_ascii_lowercase().as_str() {
                "cpp" => FileExtensions::Cpp,
                "c" => FileExtensions::C,
                _ => {
                    println!("Error: Invalid file extension");
                    return;
                }
            };

            println!("\nCreating new project: {}\n", name);

            println!("    Creating directories...");

            println!("        src_dir: {}", src_dir);
            println!("        include_dir: {}", include_dir);
            println!("        build_dir: {}", build_dir);
            println!("        exec_dir: {}", exec_dir);

            std::fs::create_dir_all(format!("{}/{}", name, src_dir)).unwrap();
            std::fs::create_dir_all(format!("{}/{}", name, include_dir)).unwrap();
            std::fs::create_dir_all(format!("{}/{}", name, build_dir)).unwrap();
            std::fs::create_dir_all(format!("{}/{}", name, exec_dir)).unwrap();

            println!("    Creating files...");

            println!("        .gitignore...");
            std::fs::write(
                format!("./{}/.gitignore", &name),
                "
# C / C++
*.o
*.out
*.exe
*.dll
*.so
*.dylib
",
            )
            .unwrap();

            println!("        CMakeLists.txt...");
            std::fs::write(
                format!("./{}/CMakeLists.txt", &name),
                format!(
                    "
cmake_minimum_required(VERSION 3.24)
project({name} {project_lang})

# Set compiler flags
set(CMAKE_{project_type}STANDARD 23)
set(CMAKE_{project_type}STANDARD_REQUIRED ON)
set(CMAKE_{project_type}EXTENSIONS OFF)
set(CMAKE_{project_type}FLAGS \"${{CMAKE_{project_type}FLAGS}} -Wall -Werror -Wextra -pedantic -pedantic-errors -g\")

# Include project headers
include_directories(./{include_dir})
# Define the source files and dependencies for the executable
set(SOURCE_FILES {src_dir}/main.{extension})

# Make the project root directory the working directory when we run
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${{CMAKE_CURRENT_SOURCE_DIR}}/{exec_dir})
set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)
add_executable({name} ${{SOURCE_FILES}})
",
                    project_lang = match extension {
                        FileExtensions::Cpp => "CXX",
                        FileExtensions::C => "C",
                    },
                    project_type = match extension {
                        FileExtensions::Cpp => "CXX_",
                        FileExtensions::C => "C_",
                    },
                ),
            )
            .unwrap();

            println!("        main.{extension}...",);
            std::fs::write(
                format!("./{}/main.{extension}", &format!("{}/{}", name, src_dir),),
                format!(
                    "
{}

int main() {{
    {}
    return 0;
}}            
",
                    match extension {
                        FileExtensions::Cpp => "#include <iostream>",
                        FileExtensions::C => "#include <stdio.h>",
                    },
                    match extension {
                        FileExtensions::Cpp => "std::cout << \"Hello, world!\" << std::endl;",
                        FileExtensions::C => "printf(\"Hello, world!\\n\");",
                    },
                ),
            )
            .unwrap();

            println!("    Initializing cmake...");
            let out = Command::new("zsh")
                .arg("-c")
                .arg(format!(
                    "cmake -S {} -B {}",
                    format!("./{}/", name),
                    format!("./{}/{}/", name, build_dir)
                ))
                .output()
                .expect("Error: Failed to open project directory");

            println!("{}", String::from_utf8_lossy(&out.stdout));
            println!("{}", String::from_utf8_lossy(&out.stderr));

            if out.status.code().unwrap() != 0 {
                println!("Error: Failed to initialize cmake");
                return;
            }

            println!("    Initializing git...");
            let out = Command::new("zsh")
                .arg("-c")
                .arg(format!("cd ./{}/ && git init", name))
                .output()
                .expect("Error: Failed to run git init");

            println!("{}", String::from_utf8_lossy(&out.stdout));
            println!("{}", String::from_utf8_lossy(&out.stderr));

            if out.status.code().unwrap() != 0 {
                println!("Error: Failed to initialize git");
                return;
            }

            println!("    Adding files to git...");
            let out = Command::new("zsh")
                .arg("-c")
                .arg(format!("cd ./{}/ && git add .", name))
                .output()
                .expect("Error: Failed to run git add");

            println!("{}", String::from_utf8_lossy(&out.stdout));
            println!("{}", String::from_utf8_lossy(&out.stderr));

            if out.status.code().unwrap() != 0 {
                println!("Error: Failed to add files to git");
                return;
            }

            println!("    Committing files to git...");
            let out = Command::new("zsh")
                .arg("-c")
                .arg(format!("cd {} && git commit -m \"Initial commit\"", name))
                .output()
                .expect("Error: Failed to run git commit");

            println!("{}", String::from_utf8_lossy(&out.stdout));
            println!("{}", String::from_utf8_lossy(&out.stderr));

            if out.status.code().unwrap() != 0 {
                println!("Error: Failed to commit files to git");
                return;
            }

            println!("Project created successfully!");
        }
        Commands::Init { src_dir, build_dir } => {
            println!("\nInitializing project...\n");

            let out = Command::new("zsh")
                .arg("-c")
                .arg(format!("cmake -S ./{}/ -B ./{}/", src_dir, build_dir))
                .output()
                .expect("Error: Failed to run cmake");

            println!("{}", String::from_utf8_lossy(&out.stdout));
            println!("{}", String::from_utf8_lossy(&out.stderr));

            if out.status.code().unwrap() != 0 {
                println!("Error: Failed to initialize cmake");
                return;
            }

            println!("Project initialized successfully!");
        }
        Commands::Build {
            // src_dir: _,
            // include_dir: _,
            build_dir,
            // runtime_dir: _,
            // exec_name: _,
        } => {
            println!("\nBuilding project...\n");

            let out = Command::new("zsh")
                .arg("-c")
                .arg(format!("cmake --build ./{}/", build_dir))
                .output()
                .expect("Error: Failed to run cmake");

            println!("{}", String::from_utf8_lossy(&out.stdout));
            println!("{}", String::from_utf8_lossy(&out.stderr));

            if out.status.code().unwrap() != 0 {
                println!("Error: Failed to build project");
                return;
            }

            println!("Project built successfully!");
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

            let out = Command::new("zsh")
                .arg("-c")
                .arg(format!("cmake --build {}", build_dir))
                .output()
                .expect("Error: Failed to run cmake");

            println!("{}", String::from_utf8_lossy(&out.stdout));
            println!("{}", String::from_utf8_lossy(&out.stderr));

            if out.status.code().unwrap() != 0 {
                println!("Error: Failed to build project");
                return;
            }

            println!("\nRunning project...\n");

            let out = Command::new("zsh")
                .arg("-c")
                .arg(format!(
                    "cd {exec_dir} && ./{exec_name} {exec_args}",
                    exec_dir = runtime_dir,
                    exec_name = match exec_name {
                        Some(name) => name,
                        None => Command::new("zsh")
                            .arg("-c")
                            .arg("basename $(pwd)")
                            .output()
                            .expect("Error: Failed to run basename")
                            .stdout
                            .into_iter()
                            .map(|c| c as char)
                            .collect::<String>(),
                    },
                    exec_args = args.map(|args| args.join(" ")).unwrap_or("".to_string())
                ))
                .spawn()
                .expect("Error: Failed to run executable")
                .wait_with_output()
                .expect("Error: Executable terminated with an error");

            println!("{}", String::from_utf8_lossy(&out.stdout));
            println!("{}", String::from_utf8_lossy(&out.stderr));
        }
    }
}
