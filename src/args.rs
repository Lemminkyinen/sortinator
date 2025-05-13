use clap::{Parser, ValueEnum};
use std::env::current_dir;
use std::io::Error as IoError;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum ArgError {
    PathDoesntExist(String),
    IoError(IoError),
}

impl From<IoError> for ArgError {
    fn from(value: IoError) -> Self {
        Self::IoError(value)
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum SortingTypeBy {
    Size,
    Name,
    Type,
    Age,
}

#[derive(Debug, Parser)]
pub(crate) struct Arguments {
    #[arg(short, long, value_enum, default_value_t = SortingTypeBy::Type)]
    sorting_type: SortingTypeBy,

    // TODO: Use PathBuf?
    #[arg(short, long, default_value_t = String::from("curdir"))]
    path: String,
}

impl Arguments {
    pub(crate) fn get_sorting_type(&self) -> SortingTypeBy {
        self.sorting_type.clone()
    }

    pub(crate) fn get_working_dir(&self) -> Result<PathBuf, ArgError> {
        if self.path == "curdir" {
            current_dir().map_err(ArgError::from)
        } else {
            let path = PathBuf::from(&self.path);

            // TODO: check is_dir as well
            if !path.exists() {
                return Err(ArgError::PathDoesntExist(self.path.clone()));
            }

            Ok(path)
        }
    }
}
