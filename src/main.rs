use std::env;
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use tempfile::NamedTempFile;

enum Mode {
    Record(String),
    PrintDoc,
}

fn parse_mode(args: Vec<String>) -> Result<Mode, Box<dyn Error>> {
    let args_count = args.len();

    if args_count < 2 {
        Err("Wrong amount of arguments")?;
    }

    match args[1].as_str() {
        "r" => {
            if args_count < 3 {
                Err(format!(
                    "Wrong amount of arguments for the record command: {args_count}"
                ))?
            } else {
                Ok(Mode::Record(args[2..].join(" ")))
            }
        }
        "i" => {
            let stdin = std::io::stdin();
            let mut buffer = String::new();
            stdin.lock().read_to_string(&mut buffer)?;
            Ok(Mode::Record(buffer))
        }
        "d" => Ok(Mode::PrintDoc),
        mode => Err(format!("Wrong mode: {mode}"))?,
    }
}

fn get_command_description(editor: String, placeholder: String) -> Result<String, Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;

    writeln!(file, "{placeholder}")?;
    file.flush()?;

    let status = process::Command::new(editor.as_str())
        .arg(file.path())
        .spawn()
        .expect("Failed to run the editor")
        .wait()
        .expect("VIM returned error code");

    let content = fs::read_to_string(file.path())?;

    file.close()?;
    Ok(match status.code() {
        Some(0) => content,
        _ => "".to_owned(),
    })
}

fn open_doc(file: &Path, editor: String) -> Result<(), Box<dyn Error>> {
    process::Command::new(editor.as_str())
        .arg(file)
        .spawn()
        .expect("Failed to start the editor")
        .wait()
        .expect("Vim returned error code");

    Ok(())
}

fn push_command_to_doc(doc_path: &Path, content: String) -> Result<(), Box<dyn Error>> {
    let mut file = fs::OpenOptions::new().append(true).open(doc_path)?;

    write!(file, "{content}")?;

    Ok(())
}

fn process(m: Mode, doc_path: &Path, editor: String) -> Result<(), Box<dyn Error>> {
    match m {
        Mode::Record(command) => {
            let placeholder = format!(
                "---\n\
                    <Put your description here>\n\
                    ```\n\
                    {}\n\
                    ```\n",
                command.as_str().trim()
            );

            let content = get_command_description(editor, placeholder)?;
            push_command_to_doc(doc_path, content)?;
        }
        Mode::PrintDoc => {
            open_doc(doc_path, editor)?;
        }
    }

    Ok(())
}

fn create_doc_header(doc_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(doc_path)?;

    writeln!(file, "# Useful commands and what they do.\n")?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let doc_folder = std::env::var("M_DOCPATH").map_err(|_| "Please set M_DOCPATH")?;
    let editor = std::env::var("M_EDITOR").map_or("vim".to_owned(), |e| e);

    let mode = parse_mode(args)?;

    let mut doc_path = PathBuf::new();
    doc_path.push(doc_folder);
    doc_path.push("commands.md");

    if !doc_path.exists() {
        create_doc_header(&doc_path)?;
    }

    process(mode, &doc_path, editor)?;

    Ok(())
}
