# Lumberjack
For when you want to chop large text files into even pieces.

## Use Case:
- There are times you may have a very long file, like a log, that for one reason or another you want to divide into smaller files. Depending on the number of smaller files you want, dividing manually may be incredibly time consuming. This tool allows for the quick creation of evenly-sized output files.
- The input file remains unchanged. 
- Output files are placed in a new directory named after the input file.
- Output file names are formatted `<Input File Name>_<Input File Line Start>_<Input File Line End>.<Input_File_Extension>`.

## Get the Tool
- The pre-compiled binary (for Linux and Windows) and source code are available in "Releases".
- macOS users will need to compile from source.

## Running the CLI
`$ ./lumberjack -i <INPUT FILE>`

- `--input / -i`: The name of the input file.
- `--lines / -l`: The number of lines per file, [default: 1].
- `--paragraph / -p`: Activate "Paragraph Mode" where the input file is broken up according to paragraph breaks, variable lines per file.
- `wc -l <Input File>` counts the number of lines in a file, and may be a useful command to use in conjunction with this tool.
- Ensure the program has executable permissions.

## Building from Source
Navigate to the project root directory.
- If using cargo: `$ cargo build --release`
- If not using cargo: `$ rustc -0 src/main.rs`

The executable binary should then be available in `./target/release/`

## Running the CLI from anywhere in your file system
Add the following lines to your `.bashrc` file:
```
~/.bashrc
# Lumberjack
export PATH="$PATH:/home/path/to/directory/where/this/program/lives"

alias lj="lumberjack"
```

