use crate::worksheet::Worksheet;

const WORKBOOK_HEADER: &str = include_str!("include/workbook_header.xml");

#[derive(Default, Clone)]
pub struct Workbook {
    pub worksheets: Vec<Worksheet>,
}

impl Workbook {
    pub fn new() -> Workbook {
        Workbook {
            worksheets: Vec::new(),
        }
    }

    pub fn with_worksheets(mut self, worksheets: Vec<Worksheet>) -> Workbook {
        self.worksheets = worksheets;
        self
    }

    pub fn to_xml(&self) -> String {
        format!(
            "{}\n{}</Workbook>",
            WORKBOOK_HEADER,
            self.worksheets
                .iter()
                .map(|w| (w.to_xml() + "\n"))
                .collect::<String>()
        )
    }
}
