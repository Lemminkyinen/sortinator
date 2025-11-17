mod args;
mod time_conversions;

use args::{ArgError, Arguments, SortingTypeBy};
use chrono::NaiveDate;
use clap::Parser;
use std::collections::HashMap;
use std::fs::metadata;
use std::fs::{File, create_dir, read_dir, rename};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use time_conversions::IntoNaiveDate;

// Include default types yml in the binary
const DEFAULT_YML: &str = include_str!("../file_types.yml");

fn main() -> Result<(), anyhow::Error> {
    let args = Arguments::parse();

    // Get supported file types, that will be used to sort files
    let file_types = read_yaml(args.file_type_path.as_deref())?;

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

    // For the hashmap key use a trait that implements a function that gives as a folder name
    let organized_files: HashMap<String, Vec<PathBuf>> = match sorting_type {
        SortingTypeBy::FileType => MetadataSorter::sort_by_type(&path, file_types)?,
        SortingTypeBy::CreatedAt => {
            MetadataSorter::sort_by_created_at(&path)?;
            HashMap::new()
        }
        _ => todo!(),
    };

    create_folder_organize_files(&path, organized_files)?;

    Ok(())
}

fn read_yaml(path: Option<&Path>) -> Result<HashMap<String, Vec<String>>, anyhow::Error> {
    match path {
        Some(path) => {
            let file = File::open(path)?;
            Ok(serde_yml::from_reader(file)?)
        }
        None => serde_yml::from_str(DEFAULT_YML).map_err(anyhow::Error::from),
    }
}

fn read_items(path: &Path) -> Result<Vec<PathBuf>, anyhow::Error> {
    Ok(read_dir(path)?
        .flatten()
        .filter_map(|item| {
            let path = item.path();
            if path.is_file() { Some(path) } else { None }
        })
        .collect())
}

fn create_folder_organize_files(
    og_path: &Path,
    organized_files: HashMap<String, Vec<PathBuf>>,
) -> Result<(), anyhow::Error> {
    for (file_type, files) in organized_files {
        let mut path2 = og_path.to_path_buf();
        path2.push(file_type);

        if let Err(e) = create_dir(path2.as_path())
            && e.kind() != ErrorKind::AlreadyExists
        {
            return Err(e.into());
        };

        for file in files {
            let mut folder_path = path2.clone();
            if let Some(file_name) = file.file_name() {
                folder_path.push(file_name);
                rename(file, &folder_path)?;
            }
        }
    }

    Ok(())
}

struct MetadataSorter;

impl MetadataSorter {
    /// Default sorting by day. TODO support hour, day, week, month, quarter, year
    fn sort_by_created_at(path: &Path) -> Result<(), anyhow::Error> {
        let items = read_items(path)?;

        let mut sorted_paths: HashMap<NaiveDate, Vec<PathBuf>> = HashMap::new();

        for item in items {
            let mdata = metadata(&item)?;
            let created_date = mdata.into_created_at();
            sorted_paths.entry(created_date).or_default().push(item);
        }

        for (date, paths) in sorted_paths {
            println!("For date {date}, found items {paths:?}")
        }

        Ok(())
    }

    fn sort_by_type(
        path: &Path,
        file_type_map: HashMap<String, Vec<String>>,
    ) -> Result<HashMap<String, Vec<PathBuf>>, anyhow::Error> {
        // Read all the files in the path
        let mut items = read_items(path)?;

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
        organized_files.insert(String::from("other"), other);

        Ok(organized_files)
    }
}
