mod args;

use args::{ArgError, Arguments, SortingTypeBy};
use clap::Parser;
use std::fs::read_dir;
use std::path::PathBuf;

fn main() -> Result<(), anyhow::Error> {
    let args = Arguments::parse();

    let sorting_type = args.get_sorting_type();

    // Handle path
    let path = match args.get_working_dir() {
        Ok(path) => path,
        Err(err) => {
            match err {
                ArgError::IoError(err) => {
                    println!("IO Error! {}", err)
                }
                ArgError::PathDoesntExist(path) => {
                    println!("Path '{}' doesn't exist bro!", path);
                }
            }
            return Ok(());
        }
    };

    match sorting_type {
        SortingTypeBy::Type => {
            sort_by_type(path)?;
        }
        _ => todo!(),
    }

    Ok(())
}

fn sort_by_type(path: PathBuf) -> Result<(), anyhow::Error> {
    // Read all the files in the path
    let items = read_dir(&path)?.flatten().filter_map(|item| {
        let path = item.path();
        if path.is_file() { Some(path) } else { None }
    });

    // Collection for handled types
    // TODO conf file
    let picture_types = ["jpg", "png"];
    let document_types = ["pdf", "txt"];
    let code_types = ["rs", "py"];

    let mut pictures = Vec::new();
    let mut documents = Vec::new();
    let mut code_files = Vec::new();
    let mut other = Vec::new();

    for item in items {
        match item.extension() {
            Some(ext) => {
                if picture_types.contains(&ext.to_str().unwrap()) {
                    pictures.push(item);
                    continue;
                }
                if document_types.contains(&ext.to_str().unwrap()) {
                    documents.push(item);
                    continue;
                }
                if code_types.contains(&ext.to_str().unwrap()) {
                    code_files.push(item);
                    continue;
                }
                other.push(item);
            }
            None => {
                other.push(item);
            }
        }
    }

    println!(
        "Pictures: {:?}",
        pictures
            .iter()
            .map(|i| i.file_name().unwrap())
            .collect::<Vec<_>>()
    );
    println!(
        "Documents: {:?}",
        documents
            .iter()
            .map(|i| i.file_name().unwrap())
            .collect::<Vec<_>>()
    );
    println!(
        "Code files: {:?}",
        code_files
            .iter()
            .map(|i| i.file_name().unwrap())
            .collect::<Vec<_>>()
    );
    println!(
        "Other files: {:?}",
        other
            .iter()
            .map(|i| i.file_name().unwrap())
            .collect::<Vec<_>>()
    );

    Ok(())
}
