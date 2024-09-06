use eframe::egui::Ui;

use crate::{pages::PageSelected, MyApp, Project};



pub fn project_card(ui: &mut Ui, project: &Project, app: &mut MyApp) {
  let mut title = String::from(project.name.clone());
  title.push_str(": ");
  title.push_str(&project.last_edit);
  ui.separator();
  ui.label(title.clone());
  if ui.button("open").clicked() {
      app.selected_project = project.clone();
      app.selected_page = PageSelected::ProjectView;
  }
}