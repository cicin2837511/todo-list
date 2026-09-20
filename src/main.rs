use serde::{Serialize, Deserialize};
use eframe::egui::{self, FontData, FontDefinitions, FontFamily, FontId, TextStyle, Align, Layout, Color32, Frame, Margin, RichText};
use std::fs::{self, OpenOptions};
use std::sync::Arc;

fn main() -> eframe::Result {
    eframe::run_native("My egui App", eframe::NativeOptions::default(), Box::new(|cc| Ok(Box::new(TodoList::new(cc)))))
}

#[derive(Serialize, Deserialize)]
struct Task {
    title: String
}

#[derive(Default)]
struct TodoList {
    input: String,
    tasks: Vec<Task>,
    initialized: bool
}

impl TodoList {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "JetBrainsMono Nerd Font".to_owned(),
            Arc::new(FontData::from_static(include_bytes!("../assets/fonts/JetBrainsMonoNerdFont-Regular.ttf")))
        );
        fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "JetBrainsMono Nerd Font".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.global_style_mut(|style| {
            style.text_styles.insert(TextStyle::Body, FontId::new(20.0, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Button, FontId::new(20.0, FontFamily::Proportional));
        });
        Self::default()
    }

    fn add_item(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.tasks.push(Task {
            title: self.input.clone()
        });
        self.save_to_json()?;
        Ok(())
    }

    fn remove_item(&mut self, i: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.tasks.remove(i);
        self.save_to_json()?;
        Ok(())
    }

    fn json_to_tasks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let contents = fs::read_to_string("tasks.json")?;
        self.tasks = serde_json::from_str(&contents)?;
        Ok(())
    }

    fn save_to_json(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("tasks.json")?;
        serde_json::to_writer_pretty(&file, &self.tasks)?;
        Ok(())
    }

    fn list_tasks(&mut self, ui: &mut egui::Ui) -> Result<(), Box<dyn std::error::Error>> {
        let mut remove = None;
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for (i, task) in self.tasks.iter().enumerate() {
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        let x = egui::Button::new(RichText::new("X").color(Color32::from_hex("#fbf1c7").unwrap())).fill(Color32::from_hex("#cc241d").unwrap());
                        if ui.add(x).clicked() {
                            remove = Some(i);
                        }
                        Frame::new()
                            .fill(Color32::from_hex("#3c3836").unwrap())
                            .inner_margin(Margin::symmetric(6, 4))
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                    ui.label(RichText::new(&task.title).color(Color32::from_hex("#d5c4a1").unwrap()));
                                });
                            });
                    });
                }
            });
        if let Some(i) = remove {
            self.remove_item(i)?;
        }
        Ok(())
    }

    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.initialized {
            self.initialized = true;
            self.json_to_tasks()?;
        }
        Ok(())
    }

    fn render(&mut self, ui: &mut egui::Ui) -> Result<(), Box<dyn std::error::Error>> {
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| -> Result<(), Box<dyn std::error::Error>> {
            let add = egui::Button::new(RichText::new("ADD").color(Color32::from_hex("#fbf1c7").unwrap())).fill(Color32::from_hex("#689d6a").unwrap());
            if (ui.add(add).clicked() ||
               ui.input(|i| i.key_pressed(egui::Key::Enter))) &&
               !self.input.is_empty() {
                self.add_item()?;
                self.input.clear();
            }
            ui.add(
                egui::TextEdit::singleline(&mut self.input)
                .text_color(Color32::from_hex("#fbf1c7").unwrap())
                .desired_width(f32::INFINITY)
                .background_color(Color32::from_hex("#3c3836").unwrap())
                .hint_text(RichText::new("Enter task here...").color(Color32::from_hex("#bdae93").unwrap()))
            );
            Ok(())
        }).inner?;
        ui.separator();
        self.list_tasks(ui)?;
        Ok(())
    }
}

impl eframe::App for TodoList {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| -> Result<(), Box<dyn std::error::Error>> {
            self.initialize()?;
            self.render(ui)?;
            Ok(())
        });
    }
}
