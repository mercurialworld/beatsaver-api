use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BeatSaverMapSearchSort {
    FirstPublished,
    Updated,
    LastPublished,
    Created,
    Curated,
}

#[derive(Default)]
pub struct BeatSaverMapSearchBuilder {
    before: Option<DateTime<Utc>>,
    after: Option<DateTime<Utc>>,
    automapper: Option<bool>,
    page_size: Option<u32>,
    sort: Option<BeatSaverMapSearchSort>,
}

impl BeatSaverMapSearchBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn before(mut self, before: DateTime<Utc>) -> Self {
        self.before = Some(before);
        self
    }

    pub fn after(mut self, after: DateTime<Utc>) -> Self {
        self.after = Some(after);
        self
    }

    pub fn automapper(mut self, automapper: bool) -> Self {
        self.automapper = Some(automapper);
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn sort(mut self, sort: BeatSaverMapSearchSort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn build(self) -> String {
        let mut query_params = vec![];

        if let Some(before) = self.before {
            query_params.push(format!("before={}", before.to_rfc3339()));
        }

        if let Some(after) = self.after {
            query_params.push(format!("after={}", after.to_rfc3339()));
        }

        if let Some(automapper) = self.automapper {
            query_params.push(format!("automapper={}", automapper));
        }

        if let Some(page_size) = self.page_size {
            query_params.push(format!("pageSize={}", page_size));
        }

        if let Some(sort) = self.sort {
            let sort_str = match sort {
                BeatSaverMapSearchSort::FirstPublished => "FIRST_PUBLISHED",
                BeatSaverMapSearchSort::Updated => "UPDATED",
                BeatSaverMapSearchSort::LastPublished => "LAST_PUBLISHED",
                BeatSaverMapSearchSort::Created => "CREATED",
                BeatSaverMapSearchSort::Curated => "CURATED",
            };
            query_params.push(format!("sort={}", sort_str));
        }

        query_params.join("&")
    }
}
