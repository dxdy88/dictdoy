extern crate chinese_dictionary;
use chinese_dictionary::{query, WordEntry};
use std::fmt;

#[derive(Clone)]
struct MyWordEntry(WordEntry);

impl fmt::Display for MyWordEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word_entry = &self.0;
        write!(
            f, "Simplified: {}\n
                Pinyin Marks: {}\n
                English: {:?}\n
                Measure Words: {:?}\n
                HSK: {}",

                // word_entry.traditional,
                word_entry.simplified,
                word_entry.pinyin_marks,
                // word_entry.pinyin_numbers,
                word_entry.english,
                // word_entry.tone_marks,
                // word_entry.hash,
                word_entry.measure_words,
                word_entry.hsk,
                // word_entry.word_id
        )
    }
}

struct MyWordEntryAll<'a>(Vec<&'a WordEntry>);

impl fmt::Display for MyWordEntryAll<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word_entries = &self.0;
        for entry in word_entries {
            writeln!(f, "{}", MyWordEntry((*entry).clone()))?;
        }
        Ok(())
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    search_results: Vec<WordEntry>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "hello".to_owned(),
            search_results: Vec::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "custom_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/NotoSansSC-Regular.ttf")),
        );

        // Insert the font into the context
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "custom_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.menu_button("Edit", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.menu_button("Export", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(350.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(180.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Hello,");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    lorem_ipsum(ui);
                });
            });

        // use egui::text::LayoutJob;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.label);
                self.search_results = search(&self.label);
            });

            ui.separator();

            if !self.search_results.is_empty() {

                // apply text formatter later
                ui.label(MyWordEntryAll(self.search_results.iter().collect()).to_string());

                // let pixels_per_point = ui.ctx().pixels_per_point();
                // let points_per_pixel = 1.0 / pixels_per_point;
                // let extra_letter_spacing_pixels = 0.0;

                // let line_height_pixels = 30;
                // let max_rows = 10;
                // // let break_anywhere = false;
                // let overflow_character = None;

                // let text = MyWordEntryAll(self.search_results.iter().collect()).to_string();

                // egui::ScrollArea::vertical()
                // .auto_shrink(false)
                // .show(ui, |ui| {
                //     let extra_letter_spacing = points_per_pixel * *extra_letter_spacing_pixels as f32;
                //     let line_height = (*line_height_pixels != 0)
                //         .then_some(points_per_pixel * *line_height_pixels as f32);

                //     let mut job = LayoutJob::single_section(
                //         text.to_owned(),
                //         egui::TextFormat {
                //             extra_letter_spacing,
                //             line_height,
                //             ..Default::default()
                //         },
                //     );
                //     job.wrap = egui::text::TextWrapping {
                //         max_rows: *max_rows,
                //         break_anywhere: false,
                //         overflow_character: *overflow_character,
                //         ..Default::default()
                //     };

                //     // NOTE: `Label` overrides some of the wrapping settings, e.g. wrap width
                //     ui.label(job);
                //     });
            }

            else {
                blank_result(ui);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}


fn search(word: &str) -> Vec<WordEntry> {
    match query(word) {
        Some(results) => results.into_iter().cloned().collect(),
        None => Vec::new(), // Handle error appropriately
    }
}

fn lorem_ipsum(ui: &mut egui::Ui) {
    ui.with_layout(
        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
        |ui| {
            ui.add(egui::Separator::default().grow(12.0));
            ui.label(egui::RichText::new("welcome to Dictdoy,\nan experimental Chinese dictionary!").small().weak());
        },
    );
}

fn blank_result(ui: &mut egui::Ui) {
    ui.with_layout(
        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
        |ui| {
            ui.add(egui::Separator::default().grow(12.0));
            ui.label(egui::RichText::new("Search some english word").small().weak());
        },
    );
}
