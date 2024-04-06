use crate::cell::*;

#[derive(Default, Clone)]
pub struct Row {
    pub cells: Vec<Cell>,
}

impl Row {
    pub fn new() -> Row {
        Row { cells: Vec::new() }
    }

    pub fn with_cells(mut self, cells: Vec<Cell>) -> Row {
        self.cells = cells;
        self
    }

    pub fn from_cells(cells: Vec<Cell>) -> Row {
        Row::new().with_cells(cells)
    }

    pub fn to_xml(&self) -> String {
        format!(
            "<Row>\n{}</Row>",
            self.cells
                .iter()
                .map(|c| (c.to_xml() + "\n"))
                .collect::<String>()
        )
    }

    pub fn to_xml_with_style(&self, style_id: &str) -> String {
        format!(
            "<Row>\n{}</Row>",
            self.cells
                .iter()
                .map(|c| (c.to_xml_with_style(style_id) + "\n"))
                .collect::<String>()
        )
    }
}
