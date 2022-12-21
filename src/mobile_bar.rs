use crate::app::SVRaidLookup;
use crate::encounter_grid::encounter_grid;
use eframe::egui;
use eframe::egui::Context;
use sv_raid_reader::{DIFFICULTY_01, DIFFICULTY_02, DIFFICULTY_03, DIFFICULTY_04, DIFFICULTY_05, DIFFICULTY_06};

pub fn mobile_top_bar(app: &mut SVRaidLookup, ctx: &Context) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("Stars: ");
            ui.radio_value(&mut app.star_level, 1, "1");
            ui.radio_value(&mut app.star_level, 2, "2");
            ui.radio_value(&mut app.star_level, 3, "3");
            ui.radio_value(&mut app.star_level, 4, "4");
            ui.radio_value(&mut app.star_level, 5, "5");
            ui.radio_value(&mut app.star_level, 6, "6");
            app.encounters = match app.star_level {
                2 => &DIFFICULTY_02,
                3 => &DIFFICULTY_03,
                4 => &DIFFICULTY_04,
                5 => &DIFFICULTY_05,
                6 => &DIFFICULTY_06,
                _ => &DIFFICULTY_01,
            };
        });
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.menu_button(egui::RichText::new("⏷ Raids"), |ui| {
                ui.set_style(ui.ctx().style());
                egui::ScrollArea::vertical().show(ui, |ui| {
                    encounter_grid(app, ui, ctx);
                });
                if ui.ui_contains_pointer() && ui.input().pointer.any_click() {
                    ui.close_menu();
                }
            });
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
        ui.add_space(5.0);
    });
}
