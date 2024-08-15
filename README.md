# Unique Ebooks Organizer

This is a simple Rust program that scans multiple directories for PDF files (ebooks) and copies only the unique files to a newly created folder named `unique_ebooks`. The program ensures that no duplicate files are copied, even if they are found in different directories.

## Features

- Scans specified directories for PDF files.
- Identifies unique PDF files by name.
- Copies unique PDF files to a `unique_ebooks` folder in the current working directory.
- Prevents duplicate files from being copied.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Ensure that Rust is installed on your machine)

## How to Use

1. **Save the Program**: 
   Save the provided Rust code to a file named `main.rs`.

2. **Compile the Program**: 
   Open your terminal and navigate to the directory containing `main.rs`. Then run the following command to compile the program:

   ```bash
   rustc main.rs
   ```

   This will produce an executable file in the same directory.

3. **Run the Program**: 
   To run the program, execute the following command, replacing `/path/to/folder1`, `/path/to/folder2`, etc., with the actual paths to the directories you want to scan for ebooks:

   ```bash
   ./main /path/to/folder1 /path/to/folder2 /path/to/folder3
   ```

4. **Check the Output**:
   After running the program, check the newly created `unique_ebooks` folder in the current working directory. This folder will contain all the unique PDF files that were found across the specified directories.

## Example

If you have three folders (`ebooks1`, `ebooks2`, and `ebooks3`) that you want to scan for unique PDF files, you would run:

```bash
./main ~/Documents/ebooks1 ~/Downloads/ebooks2 ~/Desktop/ebooks3
```

The program will scan these directories, identify the unique PDF files, and copy them to the `unique_ebooks` folder.

## Notes

- The program compares files based on their names. If two different files have the same name, only one will be copied.
- If the `unique_ebooks` folder already exists, it will not be recreated, and new unique files will be added to it.
- Make sure the directories you provide contain PDF files; otherwise, the program won't find any ebooks to copy.