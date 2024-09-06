use eframe::egui::{self, Context, Ui};

use crate::{components::project_card, MyApp};



pub fn render_home_view(ui: &mut Ui, app: &mut MyApp, ctx: &Context) {
    ui.heading("Map Maker");
    ui.horizontal(|ui| {
        ui.label("your projects");
        ui.add_space(8.);
        ui.label("|");
        ui.add_space(10.);
        if ui.button("new project").clicked() {
            app.show_new_project = true;
        }
    });
    ui.vertical(|ui| {
        for project in app.projects.clone() {
            project_card(ui, &project, app);
        }
    });

  if app.show_new_project {
    egui::Window::new("New Project")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("project name");
                ui.text_edit_singleline(&mut app.new_project.name);
            });
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Close").clicked() {
                    app.show_new_project = false;
                    {
    
                    }}
                if ui.button("Create").clicked() {
                    app.add_project();
                    app.show_new_project = false;
                }
            })
        });
  }
}