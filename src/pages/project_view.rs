use eframe::egui::Ui;

use crate::MyApp;

use super::PageSelected;

pub fn render_project_view(ui: &mut Ui, app: &mut MyApp) {
  let mut title = "Project: ".to_string();
  title.push_str(&app.selected_project.name);

  if ui.button("go back").clicked() {
    app.selected_page = PageSelected::Home;
  }
  ui.heading(title);
}