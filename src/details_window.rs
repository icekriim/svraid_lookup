use eframe::egui::Context;
#[allow(unused_imports)]
use egui_extras::RetainedImage;
use std::sync::{Arc, Mutex};
#[allow(unused_imports)]
use sv_raid_reader::{
    personal_table, ExtraActionTrigger, ExtraActionType, GemType, Gender, ItemSubject, ItemTable,
    IvType, PersonalInfo, RaidEncounter, Seikaku, ShinyType, Tokusei, ABILITIES, ITEMS,
    LOTTERY_ITEMS, NATURES, SPECIES, TYPES,
};

#[derive(Clone)]
pub struct DetailsWindow {
    pub species: String,
    pub level: String,
    pub stars: String,
    pub ability: String,
    pub nature: String,
    pub gender: String,
    pub hp: String,
    pub base_stats: String,
    pub base_type: String,
    pub image: Arc<Mutex<Option<RetainedImage>>>,
}

impl DetailsWindow {
    pub fn new(encounter: &RaidEncounter, ctx: &Context) -> Self {
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
                    .get_form_entry(encounter.species as usize, encounter.form as usize)
                    .get_ability_index(0)
                    .unwrap()]
            }
            Tokusei::Set2 => {
                ABILITIES[personal_table::SV
                    .get_form_entry(encounter.species as usize, encounter.form as usize)
                    .get_ability_index(1)
                    .unwrap()]
            }
            Tokusei::Set3 => {
                ABILITIES[personal_table::SV
                    .get_form_entry(encounter.species as usize, encounter.form as usize)
                    .get_ability_index(2)
                    .unwrap()]
            }
        };

        let nature = match encounter.seikaku {
            Seikaku::Random => "Random",
            i => NATURES[i as usize - 1],
        };

        let gender = match encounter.gender {
            Gender::Random => "Random",
            Gender::Male => "Male",
            Gender::Female => "Female",
        };

        let mut base_stats = personal_table::SV
            .get_form_entry(encounter.species as usize, encounter.form as usize)
            .stats();

        base_stats.swap(3, 4);
        base_stats.swap(4, 5);

        let stats_str = base_stats
            .into_iter()
            .map(|i| format!("{:0>2}", i))
            .collect::<Vec<_>>()
            .join(" - ");

        let type_1 = personal_table::SV
            .get_form_entry(encounter.species as usize, encounter.form as usize)
            .get_type_1();
        let type_2 = personal_table::SV
            .get_form_entry(encounter.species as usize, encounter.form as usize)
            .get_type_2();

        let base_type = if type_1 != type_2 && type_2 < TYPES.len() {
            format!("Base Type: {}/{}", TYPES[type_1], TYPES[type_2])
        } else {
            format!("Base Type: {}", TYPES[type_1])
        };

        let image_url = format!("https://raw.githubusercontent.com/Lincoln-LM/sv-live-map/master/resources/sprites/{}{}.png", encounter.species, if encounter.form != 0 { format!("-{}", encounter.form) } else { "".to_string() });

        let image = Arc::new(Mutex::new(None));

        let image_request = ehttp::Request::get(image_url);

        let clone = image.clone();
        let ctx = ctx.clone();
        ehttp::fetch(image_request, move |response| {
            if let Ok(response) = response {
                if let Ok(image) = RetainedImage::from_image_bytes(&response.url, &response.bytes) {
                    let mut lock = clone.lock().unwrap();
                    *lock = Some(image);
                    ctx.request_repaint();
                }
            }
        });

        Self {
            species: format!("Species: {}", SPECIES[encounter.species as usize]),
            level: format!("Raid Level: {}", encounter.level),
            stars: format!("Stars: {}", encounter.difficulty),
            ability: format!("Ability: {}", ability),
            nature: format!("Nature: {}", nature),
            gender: format!("Gender: {}", gender),
            hp: format!("HP: {}", encounter.hp_coef),
            base_stats: format!("Base Stats: {}", stats_str),
            base_type,
            image,
        }
    }
}
