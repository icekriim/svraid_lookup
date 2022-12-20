use crate::details_window::DetailsWindow;
use eframe::egui::{Context, DroppedFile, Vec2, Visuals, Widget};
use eframe::{egui, App, CreationContext, Frame};
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;
use std::sync::{Arc, Mutex};
use sv_raid_reader::{
    ItemTable, RaidEncounter, DIFFICULTY_01, DIFFICULTY_02, DIFFICULTY_03, DIFFICULTY_04,
    DIFFICULTY_05, DIFFICULTY_06, SPECIES,
};

pub struct SVRaidLookup {
    star_level: u8,
    species_filter: String,
    encounters: &'static [RaidEncounter],
    event_encounters: Arc<Mutex<Vec<RaidEncounter>>>,
    fixed_event_item: Arc<Mutex<ItemTable>>,
    lottery_event_items: Arc<Mutex<ItemTable>>,
    details_window: Option<DetailsWindow>,
}

impl Default for SVRaidLookup {
    fn default() -> Self {
        Self {
            star_level: 1,
            species_filter: String::new(),
            encounters: &DIFFICULTY_01,
            event_encounters: Arc::new(Mutex::new(vec![])),
            fixed_event_item: Arc::new(Mutex::new(ItemTable(HashMap::default()))),
            lottery_event_items: Arc::new(Mutex::new(ItemTable(HashMap::default()))),
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
            ui.add_space(15.0);
            ui.vertical_centered_justified(|ui| {
                if ui.button("Load Latest Event Data").clicked() {
                    let request = ehttp::Request::get("https://citrusbolt.net/bcat/v/latest/raid/files/raid_enemy_array");
                    let clone = self.event_encounters.clone();
                    ehttp::fetch(request, move |response| {
                        if let Ok(response) = response {
                            if let Ok(raid_table_array) = sv_raid_reader::delivery_enemy_table_generated::root_as_delivery_raid_enemy_table_array(&response.bytes) {
                                if let Ok(mut event_encounters) = clone.lock() {
                                    *event_encounters = raid_table_array.values().into_iter().map(|t| t.raidEnemyInfo().into()).collect::<Vec<_>>();
                                }
                            }
                        }
                    });

                    let request = ehttp::Request::get("https://citrusbolt.net/bcat/v/latest/raid/files/fixed_reward_item_array");
                    let clone = self.fixed_event_item.clone();
                    ehttp::fetch(request, move |response| {
                        if let Ok(response) = response {
                            if let Ok(fixed_item_table) = sv_raid_reader::raid_fixed_reward_item_generated::root_as_raid_fixed_reward_item_array(&response.bytes) {
                                if let Ok(mut fixed_event_items) = clone.lock() {
                                    *fixed_event_items = fixed_item_table.into();
                                }
                            }
                        }
                    });

                    let request = ehttp::Request::get("https://citrusbolt.net/bcat/v/latest/raid/files/lottery_reward_item_array");
                    let clone = self.lottery_event_items.clone();
                    ehttp::fetch(request, move |response| {
                        if let Ok(response) = response {
                            if let Ok(lottery_item_table) = sv_raid_reader::raid_lottery_reward_item_generated::root_as_raid_lottery_reward_item_array(&response.bytes) {
                                if let Ok(mut lottery_event_items) = clone.lock() {
                                    *lottery_event_items = lottery_item_table.into();
                                }
                            }
                        }
                    });
                }
            });

            ui.add_space(15.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("encounters")
                    .spacing(Vec2::new(5.0, 2.0))
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
                                        *details = DetailsWindow::new(encounter, None, None);
                                    } else {
                                        self.details_window =
                                            Some(DetailsWindow::new(encounter, None, None));
                                    }
                                }
                            });
                            if (i + 1) % 2 == 0 {
                                ui.end_row();
                            }
                        }
                        ui.end_row();
                        for (i, encounter) in self
                            .event_encounters
                            .lock()
                            .unwrap()
                            .iter()
                            .filter(|e| {
                                e.species != 0
                                    && SPECIES[e.species as usize]
                                    .to_lowercase()
                                    .contains(&self.species_filter.to_lowercase())
                            })
                            .enumerate()
                        {
                            ui.vertical_centered_justified(|ui| {
                                if ui.button(SPECIES[encounter.species as usize]).clicked() {
                                    let fixed_items = self.fixed_event_item.lock().unwrap();
                                    let lottery_items = self.lottery_event_items.lock().unwrap();
                                    if let Some(details) = self.details_window.as_mut() {
                                        *details = DetailsWindow::new(
                                            encounter,
                                            Some(&fixed_items),
                                            Some(&lottery_items),
                                        );
                                    } else {
                                        self.details_window = Some(DetailsWindow::new(
                                            encounter,
                                            Some(&fixed_items),
                                            Some(&lottery_items),
                                        ));
                                    }
                                }
                            });
                            if (i + 1) % 2 == 0 {
                                ui.end_row();
                            }
                        }
                    });
            });
        });

        if let Some(details) = self.details_window.as_ref() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    egui::Grid::new("stars_levels").show(ui, |ui| {
                        ui.label(&details.stars);
                        ui.label(&details.level);
                        ui.label(&details.shiny);
                        ui.label(&details.gender);
                        ui.label(&details.base_type);
                        ui.label(&details.base_stats);
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
                            ui.label(&details.hp);
                            ui.label(&details.nature);
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
                                if !details.second_shield_hp_trigger.is_empty() {
                                    ui.end_row();
                                    ui.label(&details.second_shield_hp_trigger);
                                    ui.label(&details.second_shield_time_trigger);
                                    ui.end_row();
                                    ui.label(&details.second_shield_damage_rate);
                                }
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
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Fixed Items:");
                            ui.add_space(5.0);
                            egui::Grid::new("fixed_items").show(ui, |ui| {
                                for (i, item) in details.fixed_items.iter().enumerate() {
                                    ui.label(item);
                                    if (i + 1) % 3 == 0 {
                                        ui.end_row();
                                    }
                                }
                            });
                        });
                        ui.add_space(50.0);
                        ui.vertical(|ui| {
                            ui.label("Random Items:");
                            ui.add_space(5.0);
                            egui::Grid::new("lottery_items").show(ui, |ui| {
                                for (i, item) in details.lottery_items.iter().enumerate() {
                                    ui.label(item);
                                    if (i + 1) % 3 == 0 {
                                        ui.end_row();
                                    }
                                }
                            });
                        });
                    });
                });
            });
        }

        if !ctx.input().raw.dropped_files.is_empty() {
            let files: Vec<DroppedFile> = ctx.input().raw.dropped_files.clone();

            for file in files.iter() {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(path) = file.path.as_ref() {
                    if let Ok(mut file) = File::open(path) {
                        let mut buf = Vec::new();
                        file.read_to_end(&mut buf).unwrap();

                        if buf.len() == 30000 {
                            if let Ok(raid_table_array) = sv_raid_reader::delivery_enemy_table_generated::root_as_delivery_raid_enemy_table_array(&buf) {
                                if let Ok(mut event_encounters) = self.event_encounters.lock() {
                                    *event_encounters = raid_table_array.values().into_iter().map(|t| t.raidEnemyInfo().into()).collect::<Vec<_>>();
                                }
                            }
                        } else if buf.len() == 27456 {
                            if let Ok(fixed_item_table) = sv_raid_reader::raid_fixed_reward_item_generated::root_as_raid_fixed_reward_item_array(&buf) {
                                if let Ok(mut fixed_event_items) = self.fixed_event_item.lock() {
                                    *fixed_event_items = fixed_item_table.into();
                                }
                            }
                        } else if buf.len() == 53464 {
                            if let Ok(lottery_item_table) = sv_raid_reader::raid_lottery_reward_item_generated::root_as_raid_lottery_reward_item_array(&buf) {
                                if let Ok(mut lottery_event_items) = self.lottery_event_items.lock() {
                                    *lottery_event_items = lottery_item_table.into();
                                }
                            }
                        }
                    }
                }

                #[cfg(target_arch = "wasm32")]
                if let Some(bytes) = file.bytes.as_ref() {
                    let buf = bytes.to_vec();
                    if buf.len() == 30000 {
                        if let Ok(raid_table_array) = sv_raid_reader::delivery_enemy_table_generated::root_as_delivery_raid_enemy_table_array(&buf) {
                            if let Ok(mut event_encounters) = self.event_encounters.lock() {
                                *event_encounters = raid_table_array.values().into_iter().map(|t| t.raidEnemyInfo().into()).collect::<Vec<_>>();
                            }
                        }
                    } else if buf.len() == 27456 {
                        if let Ok(fixed_item_table) = sv_raid_reader::raid_fixed_reward_item_generated::root_as_raid_fixed_reward_item_array(&buf) {
                            if let Ok(mut fixed_event_items) = self.fixed_event_item.lock() {
                                *fixed_event_items = fixed_item_table.into();
                            }
                        }
                    } else if buf.len() == 53464 {
                        if let Ok(lottery_item_table) = sv_raid_reader::raid_lottery_reward_item_generated::root_as_raid_lottery_reward_item_array(&buf) {
                            if let Ok(mut lottery_event_items) = self.lottery_event_items.lock() {
                                *lottery_event_items = lottery_item_table.into();
                            }
                        }
                    }
                }
            }
        }
    }
}
