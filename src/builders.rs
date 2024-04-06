use is_url::is_url;
use std::fs;

use crate::{convert::*, Worksheet};

#[derive(Debug, Clone, PartialEq)]
pub enum BuilderTableSource {
    Csv(String),
    Directory(String),
    Rss(String),
}

impl BuilderTableSource {
    pub fn resolve(&self) -> Result<String, String> {
        match self {
            Self::Csv(path) | Self::Rss(path) => {
                if is_url(path) {
                    ureq::get(path)
                        .call()
                        .map_err(|e| format!("url: {}", e))?
                        .into_string()
                        .map_err(|e| format!("url: {}", e))
                } else {
                    fs::read_to_string(path).map_err(|e| format!("file: {}", e))
                }
            }
            Self::Directory(path) => Ok(path.clone()),
        }
    }

    pub fn string(&self) -> String {
        match self {
            Self::Csv(s) => s.clone(),
            Self::Directory(s) => s.clone(),
            Self::Rss(s) => s.clone(),
        }
    }

    pub fn string_mut(&mut self) -> &mut String {
        match self {
            Self::Csv(s) => s,
            Self::Directory(s) => s,
            Self::Rss(s) => s,
        }
    }

    pub fn set_string(&mut self, str_in: String) {
        match self {
            Self::Csv(s) => *s = str_in,
            Self::Directory(s) => *s = str_in,
            Self::Rss(s) => *s = str_in,
        }
    }
}

#[derive(Default, Clone)]
pub struct BuilderWorksheet {
    pub table_source: Option<BuilderTableSource>,
    pub title: String,
    pub headings: bool,
}

impl BuilderWorksheet {
    pub fn new() -> BuilderWorksheet {
        BuilderWorksheet {
            table_source: None,
            title: String::new(),
            headings: true,
        }
    }
}

#[derive(Default)]
pub struct Builder {
    pub worksheets: Vec<BuilderWorksheet>,
    pub output: Option<String>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            worksheets: Vec::new(),
            output: None,
        }
    }

    pub fn build(&self) -> Result<Vec<Worksheet>, String> {
        let mut worksheets: Vec<Worksheet> = Vec::new();

        for w in self.worksheets.clone() {
            let src = w.table_source.unwrap();

            let src_content = match src.resolve() {
                Ok(s) => s,
                Err(e) => return Err(e),
            };

            let table = match src {
                BuilderTableSource::Csv(_) => match csv_to_table(&src_content, w.headings) {
                    Ok(t) => t,
                    Err(e) => return Err(format!("csv: {}", e)),
                },
                BuilderTableSource::Directory(_) => directory_to_table(&src_content, w.headings),
                BuilderTableSource::Rss(_) => match rss_to_table(&src_content, w.headings) {
                    Ok(t) => t,
                    Err(e) => return Err(format!("rss: {}", e)),
                },
            };

            worksheets.push(Worksheet::new().with_name(&w.title).with_table(table));
        }

        Ok(worksheets)
    }
}
