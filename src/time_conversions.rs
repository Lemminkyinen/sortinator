use chrono::{DateTime, NaiveDate, Utc};
use std::{fs::Metadata, time::SystemTime};

pub(crate) trait IntoNaiveDate {
    fn from_system_time(t: SystemTime) -> NaiveDate {
        let dt: DateTime<Utc> = t.into();
        dt.date_naive()
    }

    fn into_created_at(self) -> NaiveDate;
    fn into_accessed_at(self) -> NaiveDate;
    fn into_modified_at(self) -> NaiveDate;
}

impl IntoNaiveDate for Metadata {
    fn into_created_at(self) -> NaiveDate {
        let created = self
            .created()
            .expect("Failed to fetch created at from metadata!");
        Self::from_system_time(created)
    }
    fn into_accessed_at(self) -> NaiveDate {
        let accessed = self
            .accessed()
            .expect("Failed to fetch accessed at from metadata!");
        Self::from_system_time(accessed)
    }
    fn into_modified_at(self) -> NaiveDate {
        let modified = self
            .modified()
            .expect("Failed to fetch modified at from metadata!");
        Self::from_system_time(modified)
    }
}
