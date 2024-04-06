use std::fmt;

use crate::xml;

#[derive(Clone)]
pub enum Cell {
    Text(String),
    Number(f64),
    Empty,
}

impl Cell {
    pub const fn xml_type_str(&self) -> &'static str {
        match self {
            Cell::Text(_) => "String",
            Cell::Number(_) => "Number",
            Cell::Empty => "",
        }
    }

    pub fn to_xml(&self) -> String {
        format!(
            "<Cell><Data ss:Type=\"{}\">{}</Data></Cell>",
            self.xml_type_str(),
            xml::escape_string(self.to_string().as_str())
        )
    }

    pub fn to_xml_with_style(&self, style_id: &str) -> String {
        format!(
            "<Cell ss:StyleID=\"{}\"><Data ss:Type=\"{}\">{}</Data></Cell>",
            style_id,
            self.xml_type_str(),
            xml::escape_string(self.to_string().as_str())
        )
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Text(v) => v.to_string(),
                Cell::Number(v) => v.to_string(),
                Cell::Empty => "".to_string(),
            },
        )
    }
}
