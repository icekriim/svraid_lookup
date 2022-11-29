use std::time::Duration;
use sv_raid_reader::{
    personal_table, ExtraActionTrigger, ExtraActionType, GemType, IvType, PersonalInfo,
    RaidEncounter, Tokusei, ABILITIES, MOVES, SPECIES,
};

#[derive(Clone)]
pub struct DetailsWindow {
    pub species: String,
    pub level: String,
    pub stars: String,
    pub moves: [String; 4],
    pub gem_type: String,
    pub ability: String,
    pub flawless_ivs: String,
    pub iv_type: String,
    pub ivs: String,
    pub evs: String,
    pub hp: String,
    pub shield_hp_trigger: String,
    pub shield_time_trigger: String,
    pub shield_cancel_damage: String,
    pub shield_damage_rate: String,
    pub shield_gem_damage_rate: String,
    pub shield_change_gem_damage_rate: String,
    pub extra_actions: [String; 6],
    pub raid_time: String,
    pub command_time: String,
}

impl DetailsWindow {
    pub fn new(encounter: &RaidEncounter) -> Self {
        let gem_type = match encounter.gem_type {
            GemType::Normal => "Normal",
            GemType::Fighting => "Fighting",
            GemType::Flying => "Flying",
            GemType::Poison => "Poison",
            GemType::Ground => "Ground",
            GemType::Rock => "Rock",
            GemType::Bug => "Bug",
            GemType::Ghost => "Ghost",
            GemType::Steel => "Steel",
            GemType::Fire => "Fire",
            GemType::Water => "Water",
            GemType::Grass => "Grass",
            GemType::Electric => "Electric",
            GemType::Psychic => "Psychic",
            GemType::Ice => "Ice",
            GemType::Dragon => "Dragon",
            GemType::Dark => "Dark",
            GemType::Fairy => "Fairy",
            GemType::Random => "Random",
        };

        let ability = match encounter.tokusei {
            Tokusei::Random12 => "Random 1/2",
            Tokusei::Random123 => "Random 1/2/H",
            Tokusei::Set1 => {
                ABILITIES[personal_table::SV
                    .get_form_entry(encounter.species as usize, 0)
                    .get_ability_index(0)
                    .unwrap()]
            }
            Tokusei::Set2 => {
                ABILITIES[personal_table::SV
                    .get_form_entry(encounter.species as usize, 0)
                    .get_ability_index(1)
                    .unwrap()]
            }
            Tokusei::Set3 => {
                ABILITIES[personal_table::SV
                    .get_form_entry(encounter.species as usize, 0)
                    .get_ability_index(2)
                    .unwrap()]
            }
        };

        let iv_type = match encounter.iv_type {
            IvType::Random => "Random",
            IvType::VNum => "Variable Flawless",
            IvType::Value => "Set",
        };

        let ivs = match encounter.iv_type {
            IvType::Random => "".to_string(),
            IvType::VNum => "".to_string(),
            IvType::Value => {
                let ivs = encounter
                    .ivs
                    .iter()
                    .map(|i| format!("{:0>2}", i))
                    .collect::<Vec<_>>()
                    .join("/");
                format!("IVs: {}", ivs)
            }
        };

        let evs = {
            let evs = encounter
                .evs
                .iter()
                .map(|i| format!("{:0>2}", i))
                .collect::<Vec<_>>()
                .join("/");
            format!("EVs: {}", evs)
        };

        let total_time = Duration::from_secs(encounter.game_limit as u64);
        let shield_time_trigger = total_time.as_secs_f32()
            - (total_time.as_secs_f32() * (f32::from(encounter.shield_time_trigger) / 100.0))
                .ceil();

        let mut extra_actions = [
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        for (i, action) in encounter.extra_actions.iter().enumerate() {
            let action_type = match action.action {
                ExtraActionType::None => {
                    if action.move_no.is_some() && action.value != 0 {
                        "Move"
                    } else {
                        "None"
                    }
                }
                ExtraActionType::BossStatusReset => "Boss Status Reset",
                ExtraActionType::PlayerStatusReset => "Player Status Reset",
                ExtraActionType::Move => "Move",
                ExtraActionType::GemCount => "Tera Reset",
            };
            let trigger = match action.trigger {
                ExtraActionTrigger::None => "None",
                ExtraActionTrigger::Time => "Time",
                ExtraActionTrigger::Hp => "HP",
            };
            let value = match action.trigger {
                ExtraActionTrigger::None | ExtraActionTrigger::Hp => {
                    format!("{}% HP", action.value)
                }
                ExtraActionTrigger::Time => {
                    let time = total_time.as_secs_f32()
                        - (total_time.as_secs_f32() * (f32::from(action.value) / 100.0)).ceil();
                    format!("{}s", time)
                }
            };
            let move_name = action.move_no.map(|i| MOVES[i as usize]).unwrap_or("");
            extra_actions[i] = format!(
                "Type: {} Trigger: {}\nAt: {}{}",
                action_type,
                trigger,
                value,
                if move_name.is_empty() || action.value == 0 {
                    "".to_string()
                } else {
                    format!(" Move: {}", move_name)
                }
            );
        }

        Self {
            species: format!("Species: {}", SPECIES[encounter.species as usize]),
            level: format!("Level: {}", encounter.level),
            stars: format!("Difficulty: {}", encounter.difficulty),
            moves: [
                format!(" - {}", MOVES[encounter.reusable_moves[0] as usize]),
                format!(" - {}", MOVES[encounter.reusable_moves[1] as usize]),
                format!(" - {}", MOVES[encounter.reusable_moves[2] as usize]),
                format!(" - {}", MOVES[encounter.reusable_moves[3] as usize]),
            ],
            gem_type: format!("Tera Type: {}", gem_type),
            ability: format!("Ability: {}", ability),
            flawless_ivs: format!("Flawless IVs: {}", encounter.flawless_ivs),
            iv_type: format!("IV Type: {}", iv_type),
            ivs,
            evs,
            hp: encounter.hp_coef.to_string(),
            shield_hp_trigger: format!("Trigger Shield HP: {}%", encounter.shield_hp_trigger),
            shield_time_trigger: format!("Trigger Shield Time: {}s", shield_time_trigger),
            shield_cancel_damage: format!(
                "Shield Cancel Damage: {}%",
                encounter.shield_cancel_damage
            ),
            shield_damage_rate: format!("Shield Damage Rate: {}%", encounter.shield_damage_rate),
            shield_gem_damage_rate: format!(
                "Shield Gem Damage Rate: {}%",
                encounter.shield_gem_damage_rate
            ),
            shield_change_gem_damage_rate: format!(
                "Shield Change Gem Damage Rate: {}%",
                encounter.shield_gem_damage_rate
            ),
            extra_actions,
            raid_time: format!("Raid Time: {}s", encounter.game_limit),
            command_time: format!("Command Time: {}s", encounter.game_limit),
        }
    }
}
