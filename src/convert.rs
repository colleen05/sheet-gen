use std::{
    fs::{self, read_dir},
    path::{Path, PathBuf},
};

use crate::{Cell, Row, Table};

fn recurse(path: impl AsRef<Path>) -> Vec<PathBuf> {
    let Ok(entries) = read_dir(path) else {
        return vec![];
    };
    entries
        .flatten()
        .flat_map(|entry| {
            let Ok(meta) = entry.metadata() else {
                return vec![];
            };
            if meta.is_dir() {
                return recurse(entry.path());
            }
            if meta.is_file() {
                return vec![entry.path()];
            }
            vec![]
        })
        .collect()
}

pub fn directory_to_table(path: &str, headings: bool) -> Table {
    let mut table = Table::new().with_rows(
        recurse(path)
            .iter()
            .map(|p| {
                Row::from_cells(vec![
                    match p.to_str() {
                        Some(s) => Cell::Text(s.to_string()),
                        None => Cell::Empty,
                    },
                    match p.extension() {
                        Some(s) => Cell::Text(match s.to_str() {
                            Some(ext) => format!(
                                "{} (.{})",
                                match ext {
                                    "txt" => "Text".to_string(),
                                    "png" | "jpg" | "jpeg" | "bmp" | "gif" => "Image".to_string(),
                                    "mp4" | "mov" | "mkv" => "Video".to_string(),
                                    "mp3" | "wav" | "aiff" | "ogg" => "Audio".to_string(),
                                    "md" | "pdf" | "rtf" | "doc" | "docx" | "odt" | "fodt" =>
                                        "Document".to_string(),
                                    "ppt" | "pptx" | "fodp" | "odp" => "Slideshow".to_string(),
                                    "xls" | "xlsx" | "ods" | "fods" => "Spreadsheet".to_string(),
                                    "c" | "cpp" | "rs" | "js" | "py" | "xml" | "html" | "php"
                                    | "sh" | "cmd" | "bat" => "Code".to_string(),
                                    _ => "File".to_string(),
                                },
                                ext
                            ),
                            None => "File".to_string(),
                        }),
                        None => Cell::Text("File".to_string()),
                    },
                    match fs::metadata(p) {
                        Ok(m) => Cell::Number(m.len() as f64),
                        _ => Cell::Empty,
                    },
                ])
            })
            .collect(),
    );

    if headings {
        table.headings = Some(vec![
            "File".to_string(),
            "Type".to_string(),
            "Size (bytes)".to_string(),
        ]);
    }

    table
}

pub fn rss_to_table(text: &str, headings: bool) -> Result<Table, rss::Error> {
    let channel = rss::Channel::read_from(text.as_bytes())?;
    let mut table = Table::new().with_rows(
        channel
            .items
            .iter()
            .map(|item| {
                Row::from_cells(vec![
                    match item.pub_date.clone() {
                        Some(s) => Cell::Text(s),
                        None => Cell::Empty,
                    },
                    match item.title.clone() {
                        Some(s) => Cell::Text(s),
                        None => Cell::Empty,
                    },
                    match item.description.clone() {
                        Some(s) => Cell::Text(s),
                        None => Cell::Empty,
                    },
                    match item.link.clone() {
                        Some(s) => Cell::Text(s),
                        None => Cell::Empty,
                    },
                ])
            })
            .collect(),
    );

    if headings {
        table.headings = Some(vec![
            "Date".to_string(),
            "Title".to_string(),
            "Description".to_string(),
            "Link".to_string(),
        ]);
    }

    Ok(table)
}

pub fn csv_to_table(text: &str, headings: bool) -> Result<Table, csv::Error> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(headings)
        .from_reader(text.as_bytes());

    let mut table = Table::new();

    if headings {
        table.headings = Some(reader.headers()?.iter().map(|s| s.to_string()).collect());
    }

    for record in reader.records() {
        let record = record?;

        table.rows.push(Row::from_cells(
            record
                .iter()
                .map(|s| {
                    if let Ok(n) = s.parse::<f64>() {
                        Cell::Number(n)
                    } else if let Ok(n) = s.parse::<i64>() {
                        Cell::Number(n as f64)
                    } else {
                        Cell::Text(s.to_string())
                    }
                })
                .collect::<Vec<Cell>>(),
        ));
    }

    Ok(table)
}
