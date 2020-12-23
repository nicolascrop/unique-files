# Unique files

## Description
Rust windows GUI project done with https://github.com/rust-qt. Browse all files from different sources folder and ensure there is only
one occurrence of the source file in the target folder.

# How to use
Install the program with msi installer in the dist folder

## Build

1. Install Qt 5.15 - 5.9
2. Install Windows SDK
3. Use x86_x64 Cross Tools Command Prompt and go the project directory
4. Debug: `cargo run`
5. Release: `cargo rustc --release -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"`
