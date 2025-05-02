mod args;

use args::{Arguments, SortingTypeBy};
use clap::Parser;
use std::{env::current_dir, path::Path};

fn main() -> Result<(), anyhow::Error> {
    let args = Arguments::parse();
    println!("args: {:?}", args);

    let sorting_type = args.get_sorting_type();
    let path = current_dir()?;

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
