use clap::{Parser, ValueEnum};

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
}

impl Arguments {
    pub(crate) fn get_sorting_type(self) -> SortingTypeBy {
        self.sorting_type
    }
}
