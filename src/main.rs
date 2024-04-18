use clap::{Parser, Subcommand};
use std::{fmt::Display, fs, process::Command};

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
        /// Sets the root directory
        #[clap(short, long, default_value = ".")]
        root_dir: String,

        /// Sets the build directory
        #[clap(short, long, default_value = "build")]
        build_dir: String,
    },
    /// Builds the C++ project
    Build {
        /// Sets the build directory
        #[clap(short, long, default_value = "build")]
        build_dir: String,
    },
    /// Runs the built C++ project
    Run {
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
        #[clap(last = true)]
        args: Vec<String>,
    },
}

enum FileExtension {
    Cpp,
    C,
}

impl Display for FileExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileExtension::Cpp => write!(f, "cpp"),
            FileExtension::C => write!(f, "c"),
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
        } => handle_new_project(name, file_ext, src_dir, include_dir, build_dir, exec_dir),
        Commands::Init {
            root_dir,
            build_dir,
        } => handle_init_project(root_dir, build_dir),
        Commands::Build { build_dir } => handle_build_project(build_dir),
        Commands::Run {
            build_dir,
            runtime_dir,
            exec_name,
            args,
        } => handle_run_project(build_dir, runtime_dir, exec_name, args),
    }
}

fn handle_new_project(
    name: String,
    file_ext: String,
    src_dir: String,
    include_dir: String,
    build_dir: String,
    exec_dir: String,
) {
    if fs::metadata(&name).is_ok() {
        println!("Error: Project already exists");
        return;
    }

    let file_ext = match file_ext.to_ascii_lowercase().as_str() {
        "cpp" => FileExtension::Cpp,
        "c" => FileExtension::C,
        _ => {
            println!("Error: Invalid file extension");
            return;
        }
    };

    create_directories(&name, &src_dir, &include_dir, &build_dir, &exec_dir);
    create_project_files(&name, &src_dir, &include_dir, &exec_dir, &file_ext);
    initialize_version_control(&name);
    handle_init_project(name, build_dir);
}

fn create_directories(
    name: &str,
    src_dir: &str,
    include_dir: &str,
    build_dir: &str,
    exec_dir: &str,
) {
    fs::create_dir_all(format!("{}/{}", name, src_dir)).unwrap();
    fs::create_dir_all(format!("{}/{}", name, include_dir)).unwrap();
    fs::create_dir_all(format!("{}/{}", name, build_dir)).unwrap();
    fs::create_dir_all(format!("{}/{}", name, exec_dir)).unwrap();
}

fn create_project_files(
    name: &str,
    src_dir: &str,
    include_dir: &str,
    exec_dir: &str,
    file_ext: &FileExtension,
) {
    let project_lang = match file_ext {
        FileExtension::Cpp => "CXX",
        FileExtension::C => "C",
    };

    let project_type = match file_ext {
        FileExtension::Cpp => "CXX_",
        FileExtension::C => "C_",
    };

    let version = match file_ext {
        FileExtension::Cpp => "23",
        FileExtension::C => "17",
    };

    fs::write(
        format!("{}/.gitignore", name),
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

    fs::write(
        format!("{}/CMakeLists.txt", name),
        format!(
            "
cmake_minimum_required(VERSION 3.24)
project({name} {project_lang})

# Set compiler flags
set(CMAKE_{project_type}STANDARD {version})
set(CMAKE_{project_type}STANDARD_REQUIRED ON)
set(CMAKE_{project_type}EXTENSIONS OFF)
set(CMAKE_{project_type}FLAGS \"${{CMAKE_{project_type}FLAGS}} -Wall -Werror -Wextra -pedantic -pedantic-errors -g\")

# Include project headers
include_directories(./{include_dir})
# Define the source files and dependencies for the executable
set(SOURCE_FILES {src_dir}/main.{file_ext})

# Make the project root directory the working directory when we run
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${{CMAKE_CURRENT_SOURCE_DIR}}/{exec_dir})
set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)
add_executable({name} ${{SOURCE_FILES}})
",
        ),
    ).unwrap();

    fs::write(
        format!("{}/{}/main.{}", name, src_dir, file_ext),
        format!(
            "
{}
int main() {{
    {}
    return 0;
}}
",
            match file_ext {
                FileExtension::Cpp => "#include <iostream>",
                FileExtension::C => "#include <stdio.h>",
            },
            match file_ext {
                FileExtension::Cpp => "std::cout << \"Hello, world!\" << std::endl;",
                FileExtension::C => "printf(\"Hello, world!\\n\");",
            },
        ),
    )
    .unwrap();
}

fn initialize_version_control(name: &str) {
    let command = format!(
        "cd {} && git init && git add . && git commit -m \"Initial commit\"",
        name
    );

    run_command(&command);
}

fn handle_init_project(root_dir: String, build_dir: String) {
    let command = format!("cmake -S ./{}/ -B ./{}/{}/", root_dir, root_dir, build_dir);

    run_command(&command);
}

fn handle_build_project(build_dir: String) {
    let command = format!("cmake --build ./{}/", build_dir);

    run_command(&command);
}

fn handle_run_project(
    build_dir: String,
    runtime_dir: String,
    exec_name: Option<String>,
    args: Vec<String>,
) {
    let exec_name = exec_name.unwrap_or_else(|| {
        let output = Command::new("pwd")
            .output()
            .expect("Failed to execute command");
        let pwd = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let name = pwd.split('/').last().unwrap();
        name.to_string()
    });
    let args = args.join(" ");
    let command = format!("cd {} && ./{} {}", runtime_dir, exec_name, args);

    handle_build_project(build_dir.clone());
    run_command(&command);
}

fn run_command(command: &str) {
    let output = Command::new("zsh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to execute command")
        .wait_with_output()
        .expect("Command terminated with error");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("Error: Command execution failed");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
