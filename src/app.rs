use crate::details_window::DetailsWindow;
use crate::is_mobile;
use crate::mobile_bar::mobile_top_bar;
use crate::side_panel::draw_side_panel;
use eframe::egui::{Context, DroppedFile, Visuals};
use eframe::{egui, App, CreationContext, Frame};
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;
use std::sync::{Arc, Mutex};
use sv_raid_reader::{ItemTable, RaidEncounter, DIFFICULTY_06, SPECIES};

pub struct SVRaidLookup {
    pub star_level: u8,
    pub species_filter: String,
    pub encounters: Vec<RaidEncounter>,
    pub event_encounters: Arc<Mutex<Vec<RaidEncounter>>>,
    pub fixed_event_item: Arc<Mutex<ItemTable>>,
    pub lottery_event_items: Arc<Mutex<ItemTable>>,
    pub details_window: Option<DetailsWindow>,
}

impl Default for SVRaidLookup {
    fn default() -> Self {
        Self {
            star_level: 6,
            species_filter: String::new(),
            encounters: {
                let mut enc = DIFFICULTY_06.to_vec();
                enc.sort_by_key(|e| SPECIES[e.species as usize]);
                enc
            },
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
        if is_mobile(ctx) {
            mobile_top_bar(self, ctx);
        } else {
            draw_side_panel(self, ctx);
        }

        if let Some(details) = self.details_window.as_ref() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let image = details.image.lock().unwrap();
                    if let Some(image) = image.as_ref() {
                        image.show(ui);
                    }
                    ui.vertical(|ui| {
                        ui.label(&details.base_type);
                        ui.label(&details.base_stats);
                        egui::Grid::new("stars_levels").show(ui, |ui| {
                            ui.label(&details.level);
                            ui.label(&details.hp);
                            ui.end_row();
                            ui.label(&details.stars);
                            ui.label(&details.gender);
                            ui.end_row();
                            ui.label(&details.nature);
                            ui.label(&details.ability);
                        });
                    });
                });
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
                egui::ScrollArea::both().show(ui, |_ui| {
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
