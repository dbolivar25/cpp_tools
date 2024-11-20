# C++ Tools

A minimal CLI tool for managing C/C++ projects, inspired by Cargo's project management workflow. Built primarily for personal use in university coursework - feel free to use it if you find it helpful.

## Overview

This tool provides basic project management commands for C/C++ projects, handling project creation, building, and execution through a simple CLI interface. It uses CMake under the hood for build management.

## Features

- Create new C/C++ projects with a standardized directory structure
- Initialize CMake build configuration
- Build projects with standard compiler flags
- Run executables with argument support
- Format source code using clang-format
- Git initialization for new projects

## Prerequisites

- Rust toolchain (for installation)
- CMake (3.24 or later)
- C/C++ compiler (supports C17/C++23)
- Git
- clang-format (for code formatting)

## Installation

```bash
git clone https://github.com/dbolivar25/cpp_tools.git
cd cpp_tools
cargo install --path .
```

Make sure your Cargo bin directory is in your system's PATH.

## Usage

### Create a New Project

```bash
cxx new <PROJECT_NAME> [OPTIONS]
```

Options:

- `-f, --file-ext <EXT>`: File extension (c/cpp) [default: cpp]
- `-s, --src-dir <DIR>`: Source directory [default: src]
- `-i, --include-dir <DIR>`: Include directory [default: include]
- `-b, --build-dir <DIR>`: Build directory [default: build]
- `-e, --exec-dir <DIR>`: Executable directory [default: bin]

### Initialize Project

```bash
cxx init [OPTIONS]
```

Options:

- `-r, --root-dir <DIR>`: Root directory [default: .]
- `-b, --build-dir <DIR>`: Build directory [default: build]

### Build Project

```bash
cxx build [OPTIONS]
```

Options:

- `-b, --build-dir <DIR>`: Build directory [default: build]

### Run Project

```bash
cxx run [OPTIONS] [-- ARGS]
```

Options:

- `-b, --build-dir <DIR>`: Build directory [default: build]
- `-r, --runtime-dir <DIR>`: Executable directory [default: bin]
- `-e, --exec-name <NAME>`: Executable name [default: project_name]
- Arguments after `--` are passed to the executable

### Format Code

```bash
cxx format [OPTIONS]
```

Options:

- `-s, --src-dir <DIR>`: Source directory [default: src]

## Project Structure

```
project_name/
├── src/
│   └── main.cpp
├── include/
├── build/
├── bin/
├── .gitignore
└── CMakeLists.txt
```

## Build Configuration

The tool sets up projects with the following defaults:

- C++23/C17 standard
- Warning flags: -Wall -Werror -Wextra -pedantic -pedantic-errors
- Debug symbols enabled (-g)
- CMake compile commands export enabled

## License

See LICENSE file for details.
