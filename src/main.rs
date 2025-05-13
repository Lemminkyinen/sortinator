mod args;

use args::{ArgError, Arguments, SortingTypeBy};
use clap::Parser;
use std::path::Path;

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
            sort_by_type(&path)?;
            println!("Work in progress!")
        }
        _ => todo!(),
    }

    Ok(())
}

fn sort_by_type(path: &Path) -> Result<(), anyhow::Error> {
    todo!()
}
