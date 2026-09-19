use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: u32,
    title: String
}

const PATH: &str = "./src/data.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tasks: Vec<Task> = parse_json_tasks()?;
    window_loop(tasks)?;
    Ok(())
}

fn parse_json_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let json = fs::read_to_string(&PATH)?;
    if json.trim().is_empty() {
        return Ok(Vec::new());
    }
    let tasks: Vec<Task> = serde_json::from_str(&json)?;
    Ok(tasks)
}

fn tasks_add(tasks: &mut Vec<Task>, title: String) -> Result<(), Box<dyn std::error::Error>> {
    tasks.push(Task {
        id: tasks.len() as u32 + 1,
        title: title
    });
    let json = serde_json::to_string_pretty(&tasks)?;
    fs::write(PATH, json)?;
    Ok(())
}

fn tasks_remove(tasks: &mut Vec<Task>, id: u32) -> Result<(), Box<dyn std::error::Error>> {
    tasks.retain(|task| task.id != id);
    let json = serde_json::to_string_pretty(&tasks)?;
    fs::write(PATH, json)?;
    Ok(())
}

fn tasks_remove_all(tasks: &mut Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    tasks.clear();
    let json = serde_json::to_string_pretty(&tasks)?;
    fs::write(PATH, json)?;
    Ok(())
}

fn window_loop(mut tasks: Vec<Task>) -> eframe::Result {
    let mut taskname = String::new();
    eframe::run_simple_native("My Window", Default::default(), move |ctx, _frame| {
        let _ = change_font_family("/usr/share/fonts/TTF/HackNerdFont-Bold.ttf".to_string(), ctx);
        change_font_size(30.0, ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut taskname);
                if ui.button("add").clicked() && !taskname.trim().is_empty() {
                    let _ = tasks_add(&mut tasks, taskname.clone());
                    taskname.clear();
                }
            });
            if ui.button("remove all").clicked() {
                let _ = tasks_remove_all(&mut tasks);
            }
            let mut tasks_to_remove = None;
            for task in &tasks {
                ui.horizontal(|ui| {
                    ui.label(&task.title);
                    if ui.button("X").clicked() {
                        tasks_to_remove = Some(task.id);
                    }
                });
            }
            if let Some(id) = tasks_to_remove {
                let _ = tasks_remove(&mut tasks, id);
            }
        });
    })
}

fn change_font_family(path: String, ctx: &egui::Context) -> Result<(), std::io::Error> {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "font".to_owned(),
        egui::FontData::from_owned(
            std::fs::read(path)?
        ).into(),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "font".to_owned());
    ctx.set_fonts(fonts);
    Ok(())
}

fn change_font_size(size: f32, ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(size, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(size, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(30.0, egui::FontFamily::Proportional),
    );
    ctx.set_style(style);

}
