mod args;

use args::{ArgError, Arguments, SortingTypeBy};
use clap::Parser;
use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::path::{Path, PathBuf};

fn main() -> Result<(), anyhow::Error> {
    let args = Arguments::parse();

    // Get supported file types, that will be used to sort files
    let file_types = get_files_types(None)?;

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

    let organized_files = match sorting_type {
        SortingTypeBy::Type => sort_by_type(path, file_types)?,
        _ => todo!(),
    };

    // DEBUG PRINT
    for (item_type, file_extensions) in organized_files {
        println!(
            "Item type: {}, file extensions: {:?}",
            item_type, file_extensions
        );
    }

    Ok(())
}

fn read_yaml(path: &Path) -> Result<HashMap<String, Vec<String>>, anyhow::Error> {
    let file = File::open(path)?;
    Ok(serde_yml::from_reader(file)?)
}

// TODO support optional path from arguments
// TODO if not found, return default
fn get_files_types(path: Option<&Path>) -> Result<HashMap<String, Vec<String>>, anyhow::Error> {
    let yaml = read_yaml(Path::new("file_types.yml"))?;
    Ok(yaml)
}

fn sort_by_type(
    path: PathBuf,
    file_type_map: HashMap<String, Vec<String>>,
) -> Result<HashMap<String, Vec<PathBuf>>, anyhow::Error> {
    // Read all the files in the path
    let mut items = read_dir(&path)?
        .flatten()
        .filter_map(|item| {
            let path = item.path();
            if path.is_file() { Some(path) } else { None }
        })
        .collect::<Vec<_>>();

    let mut organized_files: HashMap<String, Vec<_>> = HashMap::new();

    let mut other = Vec::new();

    // Collection for handled types
    for (item_type, file_extensions) in file_type_map {
        let mut item_collection = Vec::new();

        let mut i = 0;
        while i < items.len() {
            match items[i].extension() {
                Some(ext) => {
                    if file_extensions.contains(&ext.to_string_lossy().to_string()) {
                        item_collection.push(items.remove(i));
                        continue;
                    } else {
                        // pass; i+=1;
                    }
                }
                None => {
                    other.push(items.remove(i));
                    continue;
                }
            }
            i += 1;
        }

        organized_files.insert(item_type, item_collection);
    }

    // Check items vector for unsorted items
    if !items.is_empty() {
        other.extend(items);
    }

    // Add other to organized files
    organized_files.insert(String::from("Other"), other);

    Ok(organized_files)
}
