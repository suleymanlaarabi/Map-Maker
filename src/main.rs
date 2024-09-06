use eframe::egui;
use pages::{render_selected_page, PageSelected};

mod components;
mod pages;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280., 720.]),
        ..Default::default()
    };
    eframe::run_native(
        "Map Maker",
        options,
        Box::new(|_cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Clone)]
struct Project {
    name: String,
    last_edit: String
}

impl Project {
    pub fn new(name: String, last_edit: String) -> Self {
        Self {
            name,
            last_edit
        }
    }
}
struct MyApp {
    projects: Vec<Project>,
    show_new_project: bool,
    new_project: Project,
    selected_page: PageSelected,
    selected_project: Project
}

impl MyApp {
    fn add_project(&mut self) {
        self.projects.push(self.new_project.clone());
        self.clear_current_new_project();
    }
    fn clear_current_new_project(&mut self) {
        self.new_project.name = "".to_string();
        self.new_project.last_edit = "unknown".to_string();
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {            projects: vec![
                Project::new("Hollow Knight".to_string(), "1 month".to_string()),
                Project::new("Mario Bros".to_string(), "1 month".to_string()),
                Project::new("Dead Cell".to_string(), "1 month".to_string())
            ],
            show_new_project: false,
            new_project: Project::new("".to_string(), "unknown".to_string()),
            selected_page: PageSelected::Home,
            selected_project: Project::new("unknown".to_string(), "".to_string())
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            render_selected_page(self.selected_page.clone(), ui, self, ctx);
        });
    }
}