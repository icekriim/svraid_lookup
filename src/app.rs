use crate::details_window::DetailsWindow;
use eframe::egui::{Context, Vec2, Visuals, Widget};
use eframe::{egui, App, CreationContext, Frame};
use sv_raid_reader::{
    RaidEncounter, DIFFICULTY_01, DIFFICULTY_02, DIFFICULTY_03, DIFFICULTY_04, DIFFICULTY_05,
    DIFFICULTY_06, SPECIES,
};

pub struct SVRaidLookup {
    star_level: u8,
    species_filter: String,
    encounters: &'static [RaidEncounter],
    details_window: Option<DetailsWindow>,
}

impl Default for SVRaidLookup {
    fn default() -> Self {
        Self {
            star_level: 1,
            species_filter: String::new(),
            encounters: &DIFFICULTY_01,
            details_window: None,
        }
    }
}

impl SVRaidLookup {
    pub fn new(cc: &CreationContext) -> Self {
        cc.egui_ctx.set_visuals(Visuals::default());
        Default::default()
    }
}

impl App for SVRaidLookup {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            egui::Grid::new("filters").num_columns(2).show(ui, |ui| {
                ui.label("Stars:");
                ui.vertical_centered_justified(|ui| {
                    if egui::DragValue::new(&mut self.star_level)
                        .clamp_range(1..=6)
                        .ui(ui)
                        .changed()
                    {
                        self.encounters = match self.star_level {
                            2 => &DIFFICULTY_02,
                            3 => &DIFFICULTY_03,
                            4 => &DIFFICULTY_04,
                            5 => &DIFFICULTY_05,
                            6 => &DIFFICULTY_06,
                            _ => &DIFFICULTY_01,
                        };
                    };
                });
                ui.end_row();
                ui.label("Species:");
                ui.vertical_centered_justified(|ui| {
                    egui::TextEdit::singleline(&mut self.species_filter).ui(ui);
                });
            });
        });

        if let Some(details) = self.details_window.as_ref() {
            egui::Window::new(&details.species).show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    egui::Grid::new("stars_levels").show(ui, |ui| {
                        ui.label(&details.stars);
                        ui.label(&details.level);
                        ui.end_row();
                    });
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Moves: ");
                            for mov in &details.moves {
                                ui.label(mov);
                            }
                        });
                        ui.add_space(30.0);
                        ui.vertical(|ui| {
                            ui.label(&details.iv_type);
                            if !details.ivs.is_empty() {
                                ui.label(&details.ivs);
                            } else {
                                ui.label(&details.flawless_ivs);
                            }
                            ui.label(&details.evs);
                        });
                        ui.add_space(10.0);
                        ui.vertical(|ui| {
                            egui::Grid::new("timing_details").show(ui, |ui| {
                                ui.label(&details.raid_time);
                                ui.label(&details.command_time);
                                ui.end_row();
                                ui.label(&details.shield_hp_trigger);
                                ui.label(&details.shield_time_trigger);
                                ui.end_row();
                                ui.label(&details.shield_cancel_damage);
                                ui.label(&details.shield_damage_rate);
                                ui.end_row();
                                ui.label(&details.shield_gem_damage_rate);
                                ui.label(&details.shield_change_gem_damage_rate);
                            });
                        });
                    });
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);
                    egui::Grid::new("extra_actions")
                        .spacing(Vec2::new(20.0, 10.0))
                        .show(ui, |ui| {
                            for (i, action) in details.extra_actions.iter().enumerate() {
                                ui.label(action);
                                if (i + 1) % 3 == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                    ui.add_space(15.0);
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("encounters")
                .spacing(Vec2::new(20.0, 20.0))
                .min_col_width(100.0)
                .show(ui, |ui| {
                    for (i, encounter) in self
                        .encounters
                        .iter()
                        .filter(|e| {
                            SPECIES[e.species as usize]
                                .to_lowercase()
                                .contains(&self.species_filter.to_lowercase())
                        })
                        .enumerate()
                    {
                        ui.vertical_centered_justified(|ui| {
                            if ui.button(SPECIES[encounter.species as usize]).clicked() {
                                if let Some(details) = self.details_window.as_mut() {
                                    *details = DetailsWindow::new(encounter);
                                } else {
                                    self.details_window = Some(DetailsWindow::new(encounter));
                                }
                            }
                        });
                        if (i + 1) % 5 == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    }
}
