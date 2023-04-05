use crate::app::SVRaidLookup;
use crate::encounter_grid::encounter_grid;
use eframe::egui;
use eframe::egui::{Context, Widget};
use sv_raid_reader::{
    DIFFICULTY_01, DIFFICULTY_02, DIFFICULTY_03, DIFFICULTY_04, DIFFICULTY_05, DIFFICULTY_06,
    SPECIES,
};

pub fn draw_side_panel(app: &mut SVRaidLookup, ctx: &Context) {
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        egui::Grid::new("filters").num_columns(2).show(ui, |ui| {
            ui.label("Stars:");
            ui.vertical_centered_justified(|ui| {
                if egui::DragValue::new(&mut app.star_level)
                    .clamp_range(1..=6)
                    .ui(ui)
                    .changed()
                {
                    app.encounters = match app.star_level {
                        2 => DIFFICULTY_02.to_vec(),
                        3 => DIFFICULTY_03.to_vec(),
                        4 => DIFFICULTY_04.to_vec(),
                        5 => DIFFICULTY_05.to_vec(),
                        6 => DIFFICULTY_06.to_vec(),
                        _ => DIFFICULTY_01.to_vec(),
                    };
                    app.encounters.sort_by_key(|e| SPECIES[e.species as usize]);
                };
            });
            ui.end_row();
            ui.label("Species:");
            ui.vertical_centered_justified(|ui| {
                egui::TextEdit::singleline(&mut app.species_filter).ui(ui);
            });
        });
        ui.add_space(15.0);
        ui.vertical_centered_justified(|ui| {
            if ui.button("Item Farming Raid").clicked() {
            }
        });

        ui.add_space(15.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            encounter_grid(app, ui, ctx);
        });
    });
}
