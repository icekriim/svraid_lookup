use crate::app::SVRaidLookup;
use crate::details_window::DetailsWindow;
use eframe::egui;
use eframe::egui::{Context, Ui, Vec2};
use sv_raid_reader::SPECIES;

pub fn encounter_grid(app: &mut SVRaidLookup, ui: &mut Ui, ctx: &Context) {
    egui::Grid::new("encounters")
        .spacing(Vec2::new(5.0, 2.0))
        .min_col_width(100.0)
        .show(ui, |ui| {
            for (i, encounter) in app
                .encounters
                .iter()
                .filter(|e| {
                    SPECIES[e.species as usize]
                        .to_lowercase()
                        .contains(&app.species_filter.to_lowercase())
                })
                .enumerate()
            {
                ui.vertical_centered_justified(|ui| {
                    if ui.button(SPECIES[encounter.species as usize]).clicked() {
                        if let Some(details) = app.details_window.as_mut() {
                            *details = DetailsWindow::new(encounter, ctx);
                        } else {
                            app.details_window =
                                Some(DetailsWindow::new(encounter, ctx));
                        }
                    }
                });
                if (i + 1) % 2 == 0 {
                    ui.end_row();
                }
            }
            ui.end_row();
            for (_i, _encounter) in app
                .event_encounters
                .lock()
                .unwrap()
                .iter()
                .filter(|e| {
                    e.species != 0
                        && SPECIES[e.species as usize]
                            .to_lowercase()
                            .contains(&app.species_filter.to_lowercase())
                })
                .enumerate()
            {
            }
        });
}
