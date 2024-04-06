use crate::{cell::Cell, row::*};

#[derive(Default, Clone)]
pub struct Table {
    pub headings: Option<Vec<String>>,
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            rows: Vec::new(),
            headings: None,
        }
    }

    pub fn with_rows(mut self, rows: Vec<Row>) -> Table {
        self.rows = rows;
        self
    }

    pub fn with_headings(mut self, labels: Vec<&str>) -> Table {
        self.headings = Some(labels.iter().map(|l| l.to_string()).collect());
        self
    }

    pub fn from_rows(rows: Vec<Row>) -> Table {
        Table::new().with_rows(rows)
    }

    pub fn to_xml(&self) -> String {
        format!(
            "<Table>\n{}{}</Table>",
            match self.headings.clone() {
                Some(vec) =>
                    Row::from_cells(vec.iter().map(|label| Cell::Text(label.clone())).collect())
                        .to_xml_with_style("Heading")
                        + "\n",
                None => "".to_string(),
            },
            self.rows
                .iter()
                .map(|r| (r.to_xml_with_style("Default") + "\n"))
                .collect::<String>()
        )
    }
}
