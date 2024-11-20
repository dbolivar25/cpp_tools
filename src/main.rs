use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colorize::AnsiColor;
use std::{fmt::Display, fs, process::Command};

/// A simple C/C++ project manager
#[derive(Parser)]
#[clap(version, author = "Daniel Bolivar")]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

/// Supported commands
#[derive(Subcommand)]
enum Commands {
    /// Creates a new C/C++ project
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
    /// Initializes and runs set up for the C/C++ project
    Init {
        /// Sets the root directory
        #[clap(short, long, default_value = ".")]
        root_dir: String,

        /// Sets the build directory
        #[clap(short, long, default_value = "build")]
        build_dir: String,
    },
    /// Builds the C/C++ project
    Build {
        /// Sets the build directory
        #[clap(short, long, default_value = "build")]
        build_dir: String,
    },
    /// Runs the built C/C++ project
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
    /// Formats the C/C++ project
    Format {
        /// Specifies the source directory
        #[clap(short, long, default_value = "src")]
        src_dir: String,
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

fn main() -> Result<()> {
    let Args { command } = Args::parse();

    match command {
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
        } => handle_init_project(&root_dir, &build_dir),
        Commands::Build { build_dir } => handle_build_project(build_dir),
        Commands::Run {
            build_dir,
            runtime_dir,
            exec_name,
            args,
        } => handle_run_project(build_dir, runtime_dir, exec_name, args),
        Commands::Format { src_dir } => handle_format_project(src_dir),
    }
}

fn handle_new_project(
    name: String,
    file_ext: String,
    src_dir: String,
    include_dir: String,
    build_dir: String,
    exec_dir: String,
) -> Result<()> {
    if fs::metadata(&name).is_ok() {
        anyhow::bail!("Project '{}' already exists", name);
    }

    let file_ext = match file_ext.to_ascii_lowercase().as_str() {
        "cpp" => FileExtension::Cpp,
        "c" => FileExtension::C,
        _ => {
            anyhow::bail!("Valid file extensions are 'cpp' and 'c'");
        }
    };

    create_directories(&name, &src_dir, &include_dir, &build_dir, &exec_dir)?;
    create_project_files(
        &name,
        &src_dir,
        &include_dir,
        &build_dir,
        &exec_dir,
        &file_ext,
    )?;
    handle_init_project(&name, &build_dir)?;
    initialize_version_control(&name)?;

    eprintln!("{}", format!("Created new project '{}'", name).green());

    Ok(())
}

fn create_directories(
    name: &str,
    src_dir: &str,
    include_dir: &str,
    build_dir: &str,
    exec_dir: &str,
) -> Result<()> {
    fs::create_dir_all(format!("{}/{}", name, src_dir))
        .context("Failed to create source directory")?;
    fs::create_dir_all(format!("{}/{}", name, include_dir))
        .context("Failed to create include directory")?;
    fs::create_dir_all(format!("{}/{}", name, build_dir))
        .context("Failed to create build directory")?;
    fs::create_dir_all(format!("{}/{}", name, exec_dir))
        .context("Failed to create executable directory")?;

    Ok(())
}

fn create_project_files(
    name: &str,
    src_dir: &str,
    include_dir: &str,
    build_dir: &str,
    exec_dir: &str,
    file_ext: &FileExtension,
) -> Result<()> {
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
        format!(
            "
.*

# Build and executable directories
{}
{}
",
            build_dir, exec_dir
        ),
    )
    .context("Failed to create .gitignore file")?;

    fs::write(
        format!("{}/CMakeLists.txt", name),
        format!(
            "cmake_minimum_required(VERSION 3.24)
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
    ).context("Failed to create CMakeLists.txt file")?;

    fs::write(
        format!("{}/{}/main.{}", name, src_dir, file_ext),
        format!(
            "{}

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
    .context("Failed to create main source file")?;

    Ok(())
}

fn initialize_version_control(name: &str) -> Result<()> {
    let command = format!(
        "cd {} && git init && git add . && git commit -m \"Initial commit\"",
        name
    );

    run_command(&command).context("Failed to initialize version control")?;

    Ok(())
}

fn handle_init_project(root_dir: &str, build_dir: &str) -> Result<()> {
    let command = format!("cmake -S ./{}/ -B ./{}/{}/", root_dir, root_dir, build_dir);

    run_command(&command).context("Failed to initialize project")?;

    eprintln!(
        "{}",
        format!("Initialized project in '{}'", build_dir).green()
    );

    Ok(())
}

fn handle_build_project(build_dir: String) -> Result<()> {
    let command = format!("cmake --build ./{}/", build_dir);

    run_command(&command).context("Failed to run build command")?;

    eprintln!("{}", "Build successful".green());

    Ok(())
}

fn handle_run_project(
    build_dir: String,
    runtime_dir: String,
    exec_name: Option<String>,
    args: Vec<String>,
) -> Result<()> {
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

    handle_build_project(build_dir.clone()).context("Failed to build project")?;
    run_command(&command).context("Failed to run executable")?;

    Ok(())
}

fn handle_format_project(src_dir: String) -> Result<()> {
    let command = format!("clang-format -i -style=file ./{}/{}", src_dir, "*");

    run_command(&command).context("Failed to format project")?;

    Ok(())
}

fn run_command(command: &str) -> Result<()> {
    let output = Command::new("zsh")
        .arg("-c")
        .arg(command)
        .spawn()
        .context("Failed to spawn command")?
        .wait_with_output()
        .context("Failed to wait on command")?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
