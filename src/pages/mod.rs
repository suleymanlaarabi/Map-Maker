use eframe::egui::{Context, Ui};
use home_view::render_home_view;
use project_view::render_project_view;

use crate::MyApp;

mod project_view;
mod home_view;

#[derive(Clone)]
pub enum PageSelected {
  Home,
  ProjectView,
}

pub fn render_selected_page(selected_page: PageSelected, ui: &mut Ui, app: &mut MyApp, ctx: &Context) {
  match selected_page {
      PageSelected::Home => render_home_view(ui, app, ctx),
      PageSelected::ProjectView => render_project_view(ui, app)
  }
}