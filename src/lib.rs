use eframe::egui::Context;

pub mod app;
pub mod details_window;
mod encounter_grid;
mod mobile_bar;
mod side_panel;

fn is_mobile(ctx: &Context) -> bool {
    let screen_size = ctx.input().screen_rect().size();
    screen_size.x < 550.0
}
