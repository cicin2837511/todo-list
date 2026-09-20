use serde::{Serialize, Deserialize};
use eframe::egui;
use std::fs::{self, OpenOptions};

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
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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
        for (i, task) in self.tasks.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(&task.title);
                if ui.button("X").clicked() {
                    remove = Some(i);
                }
            });
        }
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
        ui.horizontal(|ui| -> Result<(), Box<dyn std::error::Error>> {
            ui.text_edit_singleline(&mut self.input);
            if ui.button("Add Item").clicked() && !self.input.is_empty() {
                self.add_item()?;
                self.input.clear();
            }
            Ok(())
        }).inner?;
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
