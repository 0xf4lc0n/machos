use clap::{arg, command, ArgGroup, Command};
use object::{Object, ObjectSection};
use std::fs;

fn main() -> Result<(), Error> {
    let matches = command!()
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("list")
                .arg_required_else_help(true)
                .about("List MachO file sections")
                .arg(arg!(-i --input <INPUT_FILE_PATH>)),
        )
        .subcommand(
            Command::new("dump")
                .arg_required_else_help(true)
                .about("Dump selected MachO file sections")
                .args([
                    arg!(-i --input <INPUT_FILE_PATH>),
                    arg!(-s --sections <SECTIONS>),
                    arg!(-o --output <OUTPUT_FILE_PATH>),
                ])
                .group(
                    ArgGroup::new("dump_args")
                        .multiple(true)
                        .args(["input", "sections", "output"])
                        .requires_all(["input", "sections", "output"]),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("list", args)) => {
            let file_name = args
                .get_one::<String>("input")
                .expect("input file path is missing");

            if let Err(e) = print_sections(file_name) {
                println!("cannot list sections -> {e}");
            }
        }
        Some(("dump", args)) => {
            let file_name = args
                .get_one::<String>("input")
                .expect("input file path is missing");

            let sections = args
                .get_one::<String>("sections")
                .expect("sections are missing")
                .split(',')
                .map(|s| s.into())
                .collect::<Vec<String>>();

            let dump_file_path = args
                .get_one::<String>("output")
                .expect("output file path is missing");

            if let Err(e) = dump_sections(file_name, &sections, dump_file_path) {
                println!("cannot dump sections -> {e}");
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn print_sections(read_from: &str) -> Result<(), Error> {
    let binary_data = fs::read(read_from).map_err(|e| Error::ReadWrite {
        msg: format!("failed to read {read_from} file"),
        source: e,
    })?;

    let file = object::File::parse(&*binary_data)?;

    println!("Avalible sections:");
    for section in file.sections() {
        println!("{}", section.name()?);
    }

    Ok(())
}

fn dump_sections(read_from: &str, sections: &[String], write_to: &str) -> Result<(), Error> {
    let binary_data = fs::read(read_from).map_err(|e| Error::ReadWrite {
        msg: format!("failed to read {read_from} file"),
        source: e,
    })?;

    let mut dump = vec![];
    let file = object::File::parse(&*binary_data)?;

    let sections_to_dump = file
        .sections()
        .filter(|s1| sections.iter().any(|s2| s2.as_str() == s1.name().unwrap()));

    for section in sections_to_dump {
        let data = section.data()?;
        dump.extend_from_slice(data);
    }

    if dump.is_empty() {
        println!("no sections were dumped");
        return Ok(());
    }

    fs::write(write_to, dump).map_err(|e| Error::ReadWrite {
        msg: format!("failed to write {write_to} file"),
        source: e,
    })?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("{msg}")]
    ReadWrite { msg: String, source: std::io::Error },

    #[error("failed to process file")]
    MachO(#[from] object::Error),
}
