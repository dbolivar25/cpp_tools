# Cpp Tools

## Overview

This CLI tool is designed to facilitate the creation, initialization, building,
and running of C++ projects. It leverages the clap crate in Rust for command
line argument parsing, providing a user-friendly interface for managing various
aspects of a C++ project.

## Features

- Project Creation (new Command): Easily create a new C++ project with
  customizable directory structures for source, include, build, and executable
  files.
- Project Initialization (init Command): Initialize the project's build system
  and other necessary configurations.
- Project Building (build Command): Compile the project into an executable.
- Project Execution (run Command): Run the compiled project executable with
  optional arguments.

## Installation

```bash
git clone https://github.com/dbolivar25/cpp_tools.git
cd cpp_tools

cargo install --path .
```

- Ensure the cargo bin directory is a `$PATH` directory

## Usage

```bash
cxx
```
