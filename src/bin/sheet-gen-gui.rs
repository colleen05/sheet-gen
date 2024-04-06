#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release builds

use std::{env, fs};

use eframe::egui;
use egui::Ui;
use sheet_gen::{
    builders::{Builder, BuilderTableSource, BuilderWorksheet},
    Workbook,
};

const COLOUR_SUCCESS: egui::Color32 = egui::Color32::from_rgb(48, 192, 48);
const COLOUR_ERROR: egui::Color32 = egui::Color32::from_rgb(192, 48, 48);

fn main() -> Result<(), eframe::Error> {
    // Initialise logger
    env_logger::init();

    // Setup GUI options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([512.0, 640.0]),
        ..Default::default()
    };

    // Initialise app
    let mut app = SheetgenApp::default();

    if let Some(s) = env::args().nth(1) {
        app.builder.output = Some(s)
    }

    // Run app window
    eframe::run_native(
        "Spreadsheet Generator",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<SheetgenApp>::from(app)
        }),
    )
}

#[derive(Clone)]
enum AppCommand {
    None,
    MoveUp(usize),
    MoveDown(usize),
    Delete(usize),
}

struct SheetgenApp {
    builder: Builder,
    export_status: Option<Result<String, String>>,
    commands: Vec<AppCommand>,
}

impl Default for SheetgenApp {
    fn default() -> SheetgenApp {
        let mut s = SheetgenApp {
            builder: Builder::new(),
            export_status: None,
            commands: Vec::new(),
        };

        s.builder.output = Some(String::new());

        s
    }
}

impl SheetgenApp {
    pub fn worksheet_card(
        ui: &mut Ui,
        worksheet: &mut BuilderWorksheet,
        worksheet_index: usize,
    ) -> AppCommand {
        let mut command = AppCommand::None;

        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.button("❌").on_hover_text("Delete worksheet").clicked() {
                    command = AppCommand::Delete(worksheet_index);
                }

                let _ = ui
                    .text_edit_singleline(&mut worksheet.title)
                    .on_hover_text("Worksheet title");

                if ui.button("⏶").on_hover_text("Move up").clicked() {
                    command = AppCommand::MoveUp(worksheet_index);
                }

                if ui.button("⏷").on_hover_text("Move down").clicked() {
                    command = AppCommand::MoveDown(worksheet_index);
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.push_id(worksheet_index, |ui| {
                    egui::ComboBox::from_label("")
                        .selected_text(match worksheet.table_source.clone().unwrap() {
                            BuilderTableSource::Csv(_) => "CSV",
                            BuilderTableSource::Directory(_) => "Directory",
                            BuilderTableSource::Rss(_) => "RSS",
                        })
                        .show_ui(ui, |ui| {
                            let src_string = worksheet.table_source.clone().unwrap().string();

                            ui.selectable_value(
                                &mut worksheet.table_source,
                                Some(BuilderTableSource::Csv(src_string.clone())),
                                "CSV",
                            );
                            ui.selectable_value(
                                &mut worksheet.table_source,
                                Some(BuilderTableSource::Directory(src_string.clone())),
                                "Directory",
                            );
                            ui.selectable_value(
                                &mut worksheet.table_source,
                                Some(BuilderTableSource::Rss(src_string.clone())),
                                "RSS",
                            );
                        });
                });

                let _ = ui
                    .text_edit_singleline(worksheet.table_source.as_mut().unwrap().string_mut())
                    .on_hover_text("Source path / URL");

                ui.checkbox(&mut worksheet.headings, "Headings")
            });
        });

        ui.add_space(8.0);

        command
    }

    pub fn export(&mut self) {
        // Generate worksheets
        let worksheets = match self.builder.build() {
            Ok(v) => v,
            Err(e) => {
                self.export_status = Some(Err(e));
                return;
            }
        };

        // Create workbook and export or print
        let workbook = Workbook::new().with_worksheets(worksheets);
        let workbook_xml = workbook.to_xml();

        match &self.builder.output {
            Some(p) => {
                if let Err(e) = fs::write(p, workbook_xml) {
                    self.export_status = Some(Err(e.to_string()));
                    return;
                };

                self.export_status = Some(Ok(format!("Success! Exported to \"{}\".", p)));
            }
            None => println!("{}", workbook_xml),
        }
    }

    pub fn process_commands(&mut self) {
        for cmd in self.commands.clone() {
            match cmd {
                AppCommand::None => {}
                AppCommand::Delete(i) => {
                    self.builder.worksheets.remove(i);
                }
                AppCommand::MoveUp(i) => {
                    if i < self.builder.worksheets.len() && i > 0 {
                        let temp = self.builder.worksheets.remove(i);
                        self.builder.worksheets.insert(i - 1, temp);
                    }
                }
                AppCommand::MoveDown(i) => {
                    if i < self.builder.worksheets.len() - 1 {
                        let temp = self.builder.worksheets.remove(i);
                        self.builder.worksheets.insert(i + 1, temp);
                    }
                }
            }
        }

        self.commands.clear();
    }
}

impl eframe::App for SheetgenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui
                        .button("Clear")
                        .on_hover_text("Delete all worksheets")
                        .clicked()
                    {
                        self.builder.worksheets.clear();
                    }
                });
                ui.add_space(4.0);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, w) in self.builder.worksheets.iter_mut().enumerate() {
                    match Self::worksheet_card(ui, w, i) {
                        AppCommand::None => {}
                        cmd => self.commands.push(cmd),
                    }
                }

                if ui.button("+  Add worksheet").clicked() {
                    self.builder.worksheets.push({
                        let mut w = BuilderWorksheet::new();
                        w.table_source = Some(BuilderTableSource::Csv("data.csv".to_string()));
                        w.title = format!("Worksheet {}", self.builder.worksheets.len());
                        w
                    })
                }
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    let mut s = self.builder.output.clone().unwrap();
                    let _ = ui.text_edit_singleline(&mut s).on_hover_text("Export path");

                    if ui.button("Export").clicked() {
                        self.export_status = None;
                        self.export();
                    }

                    self.builder.output = Some(s);
                });
                if let Some(status) = self.export_status.clone() {
                    match status {
                        Ok(s) => ui.colored_label(COLOUR_SUCCESS, s),
                        Err(e) => ui.colored_label(COLOUR_ERROR, format!("Error! {}", e)),
                    };
                }
                ui.add_space(4.0);
            });

        self.process_commands();
    }
}
