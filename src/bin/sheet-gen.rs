use sheet_gen::{builders::*, *};
use std::{env, fs, process::exit};

const HELP_MESSAGE: &str = include_str!("../include/help_message");

pub enum CmdOptionType {
    None,
    WorksheetTitle,
    FromCSV,
    FromDirectory,
    FromRSS,
    OutputPath,
}

fn die(message: &str, suggest_help: bool) {
    const RED_BOLD: &str = "\x1B[1;31m";
    const BOLD: &str = "\x1B[1m";
    const RESET: &str = "\x1B[0m";

    eprintln!(
        "{}error{}: {}{}",
        RED_BOLD,
        RESET,
        message,
        if suggest_help {
            format!(
                "\n{}hint{}: try running with -h or --help for help.",
                BOLD, RESET
            )
        } else {
            String::new()
        }
    );

    exit(1);
}

fn print_help() {
    println!("{}", HELP_MESSAGE);
    exit(0);
}

fn main() {
    // Check for help
    if env::args().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
    }

    // Parse commands
    let mut builder = Builder::new();
    let mut builder_worksheet = BuilderWorksheet::new();
    let mut last_option: CmdOptionType = CmdOptionType::None;

    for (i, arg) in env::args().enumerate() {
        // Skip first command argument (always the executable path)
        if i == 0 {
            continue;
        }

        match arg.as_str() {
            "-w" => last_option = CmdOptionType::WorksheetTitle,
            "-H" => {
                last_option = CmdOptionType::None;
                builder_worksheet.headings = false;
            }
            "-c" => last_option = CmdOptionType::FromCSV,
            "-r" => last_option = CmdOptionType::FromRSS,
            "-d" => last_option = CmdOptionType::FromDirectory,
            "-o" => last_option = CmdOptionType::OutputPath,
            arg => {
                match last_option {
                    CmdOptionType::None => die("invalid syntax.", true),
                    CmdOptionType::WorksheetTitle => builder_worksheet.title = arg.to_string(),
                    CmdOptionType::OutputPath => builder.output = Some(arg.to_string()),
                    _ => {
                        builder_worksheet.table_source = Some(match last_option {
                            CmdOptionType::FromCSV => BuilderTableSource::Csv(arg.to_string()),
                            CmdOptionType::FromDirectory => {
                                BuilderTableSource::Directory(arg.to_string())
                            }
                            CmdOptionType::FromRSS => BuilderTableSource::Rss(arg.to_string()),
                            _ => unreachable!(),
                        });

                        if builder_worksheet.table_source.is_none() {
                            die("table must have a source.", true);
                        }

                        if builder_worksheet.title.is_empty() {
                            builder_worksheet.title =
                                format!("Worksheet {}", builder.worksheets.len());
                        }

                        builder.worksheets.push(builder_worksheet);
                        builder_worksheet = BuilderWorksheet::new();
                    }
                };

                last_option = CmdOptionType::None
            }
        }
    }

    // Validate
    if !matches!(last_option, CmdOptionType::None) {
        die("option given but not specified.", true);
    }

    if builder.worksheets.is_empty() {
        die("no source data given.", true)
    }

    // Generate worksheets
    let worksheets = match builder.build() {
        Ok(v) => v,
        Err(s) => {
            die(s.as_str(), false);
            unreachable!()
        }
    };

    // Create workbook and export or print
    let workbook = Workbook::new().with_worksheets(worksheets);
    let workbook_xml = workbook.to_xml();

    match builder.output {
        Some(p) => {
            if let Err(e) = fs::write(p, workbook_xml) {
                die(e.to_string().as_str(), false)
            }
        }
        None => println!("{}", workbook_xml),
    }
}
