use crate::{table::Table, xml};

#[derive(Default, Clone)]
pub struct Worksheet {
    pub name: String,
    pub table: Table,
}

impl Worksheet {
    pub fn new() -> Worksheet {
        Worksheet {
            name: String::new(),
            table: Table::new(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Worksheet {
        self.name = name.to_string();
        self
    }

    pub fn with_table(mut self, table: Table) -> Worksheet {
        self.table = table;
        self
    }

    pub fn to_xml(&self) -> String {
        format!(
            "<Worksheet ss:Name=\"{}\">\n{}\n</Worksheet>",
            xml::escape_string(self.name.as_str()),
            self.table.to_xml()
        )
    }
}
