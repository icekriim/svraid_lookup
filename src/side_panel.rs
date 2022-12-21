use crate::app::SVRaidLookup;
use crate::encounter_grid::encounter_grid;
use eframe::egui;
use eframe::egui::{Context, Widget};
use sv_raid_reader::{
    DIFFICULTY_01, DIFFICULTY_02, DIFFICULTY_03, DIFFICULTY_04, DIFFICULTY_05, DIFFICULTY_06,
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
                egui::TextEdit::singleline(&mut app.species_filter).ui(ui);
            });
        });
        ui.add_space(15.0);
        ui.vertical_centered_justified(|ui| {
            if ui.button("Load Latest Event Data").clicked() {
                let request = ehttp::Request::get("https://citrusbolt.net/bcat/v/latest/raid/files/raid_enemy_array");
                let clone = app.event_encounters.clone();
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
                let clone = app.fixed_event_item.clone();
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
                let clone = app.lottery_event_items.clone();
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
            encounter_grid(app, ui, ctx);
        });
    });
}
