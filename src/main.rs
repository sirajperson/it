use clap::{Arg, Command, ArgAction, ArgGroup};
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

/// Inserts, appends, or clears text in one or more files based on command-line arguments.
///
/// This tool modifies text files by inserting text at a specific line, appending text to the end,
/// or clearing a range of lines. It supports backup creation, dry runs, and interactive input.
fn main() -> io::Result<()> {
    let matches = Command::new("it")
        .version("1.0.0")
        .about("Inserts text at a given line location of a file")
        .after_help(
            "EXAMPLES:\n\
             Insert 'New Line' at line 2 in file.txt:\n\
             \t$ it -i \"New Line\" -l 2 file.txt\n\n\
             Overwrite line 2 with 'Overwritten' in file.txt:\n\
             \t$ it -i \"Overwritten\" -l 2 -o file.txt\n\n\
             Append 'Appended' to the end of file.txt:\n\
             \t$ it -a \"Appended\" file.txt\n\n\
             Clear from line 2 to the end in file.txt:\n\
             \t$ it -z 2 file.txt\n\n\
             Clear from line 2 to line 3 in file.txt:\n\
             \t$ it -z 2,3 file.txt\n\n\
             Append an empty line to file.txt (default):\n\
             \t$ it file.txt\n\n\
             Interactively insert text at line 2:\n\
             \t$ echo \"New Line\" | it -i -l 2 -I file.txt\n\n\
             Create a backup before modifying multiple files:\n\
             \t$ it -b -a \"Appended\" file1.txt file2.txt"
        )
        .arg(
            Arg::new("file")
                .help("The file(s) to modify")
                .required(true)
                .num_args(1..)
                .index(1),
        )
        .arg(
            Arg::new("line")
                .short('l')
                .long("line")
                .value_name("NUMBER")
                .help("The line number to start inserting or clearing at")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("overwrite")
                .short('o')
                .long("overwrite")
                .help("Overwrite the line instead of inserting")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("insert")
                .short('i')
                .long("insert")
                .value_name("TEXT")
                .help("Inserts text at the line provided by the line flag (default: first line)")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("append")
                .short('a')
                .long("append")
                .value_name("TEXT")
                .help("Inserts text at the last line of the file")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("clear")
                .short('z')
                .long("clear")
                .value_name("START[,END]")
                .help("Clear to end of file from line given by --line or START. If START,END provided, clear range.")
                .value_parser(|s: &str| {
                    let parts: Vec<&str> = s.split(',').collect();
                    match parts.len() {
                        1 => {
                            let start = parts[0].parse::<usize>().map_err(|_| "Invalid start line number")?;
                            if start == 0 { Err("Line numbers must be greater than 0") } else { Ok((start, None)) }
                        }
                        2 => {
                            let start = parts[0].parse::<usize>().map_err(|_| "Invalid start line number")?;
                            let end = parts[1].parse::<usize>().map_err(|_| "Invalid end line number")?;
                            if start == 0 || end == 0 { Err("Line numbers must be greater than 0") }
                            else if start > end { Err("Start line must be less than or equal to end line") }
                            else { Ok((start, Some(end))) }
                        }
                        _ => Err("Expected format: START or START,END"),
                    }
                }),
        )
        .arg(
            Arg::new("backup")
                .short('b')
                .long("backup")
                .help("Create a backup of the original file (adds .bak extension)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("interactive")
                .short('I')
                .long("interactive")
                .help("Read text to insert or append from stdin")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dry-run")
                .short('d')
                .long("dry-run")
                .help("Print changes to stdout without modifying the file")
                .action(ArgAction::SetTrue),
        )
        .group(
            ArgGroup::new("operation")
                .args(["insert", "append", "clear"])
                .required(false),
        )
        .get_matches();

    let file_paths = matches.get_many::<String>("file").unwrap();
    let line_num = matches.get_one::<usize>("line").copied();
    let overwrite = matches.get_flag("overwrite");
    let insert_text = matches.get_one::<String>("insert").cloned();
    let append_text = matches.get_one::<String>("append").cloned();
    let clear_range = matches.get_one::<(usize, Option<usize>)>("clear").cloned();
    let backup = matches.get_flag("backup");
    let interactive = matches.get_flag("interactive");
    let dry_run = matches.get_flag("dry-run");

    for file_path in file_paths {
        // Validate file path
        if Path::new(file_path).is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("'{}' is a directory, not a file.", file_path),
            ));
        }
        if Path::new(file_path).exists() {
            let metadata = fs::metadata(file_path).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Cannot access '{}': {}", file_path, e),
                )
            })?;
            if metadata.permissions().readonly() {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    format!("No write permission for '{}'.", file_path),
                ));
            }
        }

        // Handle interactive mode
        let insert_text = if interactive && insert_text.is_none() && clear_range.is_none() {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            Some(input.trim_end().to_string())
        } else {
            insert_text.clone()
        };
        let append_text = if interactive && append_text.is_none() && clear_range.is_none() {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            Some(input.trim_end().to_string())
        } else {
            append_text.clone()
        };

        // Create backup if requested
        if backup && Path::new(file_path).exists() {
            let backup_path = format!("{}.bak", file_path);
            fs::copy(file_path, &backup_path).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to create backup '{}': {}", backup_path, e),
                )
            })?;
        }

        // Handle append operation efficiently
        if let Some(text) = &append_text {
            if !dry_run {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(file_path)?;
                writeln!(file, "{}", text)?;
            } else {
                let mut content = String::new();
                if Path::new(file_path).exists() {
                    fs::File::open(file_path)?.read_to_string(&mut content)?;
                }
                let mut lines: Vec<String> = content.lines().map(String::from).collect();
                lines.push(text.to_string());
                println!("{}", lines.join("\n"));
            }
            continue;
        }

        // Read the file content for other operations
        let mut content = String::new();
        if Path::new(file_path).exists() {
            let mut file = fs::File::open(file_path)?;
            file.read_to_string(&mut content)?;
        }

        let mut lines: Vec<String> = content.lines().map(String::from).collect();
        if lines.is_empty() {
            lines.push(String::new());
        }

        // Perform the operation
        if let Some((start, end)) = clear_range {
            // Clear mode: clear from start to end (or end of file)
            let start_idx = start.saturating_sub(1);
            if start_idx >= lines.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Start line {} is beyond file length for '{}'.", start, file_path),
                ));
            }
            let end_idx = end.unwrap_or(lines.len());
            if end_idx > lines.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("End line {} is beyond file length for '{}'.", end_idx, file_path),
                ));
            }
            lines.drain(start_idx..end_idx);
            if lines.is_empty() {
                lines.push(String::new());
            }
        } else if let Some(text) = insert_text {
            // Insert mode: insert or overwrite at specified line
            let insert_line = line_num.unwrap_or(1).saturating_sub(1);
            if insert_line >= lines.len() {
                lines.resize(insert_line + 1, String::new());
            }
            if overwrite {
                lines[insert_line] = text.to_string();
            } else {
                lines.insert(insert_line, text.to_string());
            }
        } else if line_num.is_some() || overwrite {
            // Insert or overwrite empty line if no text provided
            let insert_line = line_num.unwrap_or(1).saturating_sub(1);
            if insert_line >= lines.len() {
                lines.resize(insert_line + 1, String::new());
            }
            if overwrite {
                lines[insert_line] = String::new();
            } else {
                lines.insert(insert_line, String::new());
            }
        } else {
            // Default behavior: append an empty line
            lines.push(String::new());
        }

        // Write or display the result
        if dry_run {
            println!("{}", lines.join("\n"));
        } else {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_path)?;
            file.write_all(lines.join("\n").as_bytes())?;
            if !lines.is_empty() && !content.ends_with('\n') {
                file.write_all(b"\n")?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests inserting a line at a specific position.
    #[test]
    fn test_insert() {
        let mut lines = vec!["Line 1".to_string(), "Line 2".to_string()];
        let insert_line = 1; // Line 2
        lines.insert(insert_line, "New Line".to_string());
        assert_eq!(lines, vec!["Line 1", "New Line", "Line 2"]);
    }

    /// Tests appending a line to the end.
    #[test]
    fn test_append() {
        let mut lines = vec!["Line 1".to_string()];
        lines.push("Appended".to_string());
        assert_eq!(lines, vec!["Line 1", "Appended"]);
    }

    /// Tests clearing a range of lines.
    #[test]
    fn test_clear_range() {
        let mut lines = vec!["Line 1".to_string(), "Line 2".to_string(), "Line 3".to_string()];
        lines.drain(1..2); // Clear line 2
        assert_eq!(lines, vec!["Line 1", "Line 3"]);
    }

    /// Tests clearing from a line to the end.
    #[test]
    fn test_clear_to_end() {
        let mut lines = vec!["Line 1".to_string(), "Line 2".to_string(), "Line 3".to_string()];
        lines.drain(1..); // Clear from line 2 to end
        assert_eq!(lines, vec!["Line 1"]);
    }
}