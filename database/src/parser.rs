use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use algocore::{Affinity, CardType, Cost, Faction, MetaCardType, ResourceType, Timing};
use crate::{CardPrototype, CardPrototypeId};
use phf::{phf_map};

const HASTE_STR: &'static str = "{Haste}";
const VIRUS_STR: &'static str = "{Virus}";
const BATTLE_STR: &'static str = "{Battle}";
const TRIGGER_STR: &'static str = "Trigger";
const STOLEN_CARD_STR: &'static str = "Stolen Card";
const SPELL_STR: &'static str = "Spell";
const TOKEN_STR: &'static str = "Token";
const META_RESOURCE_STR: &'static str = "!Resource";
const RESOURCE_STR: &'static str = "Resource";

static RESOURCE_TYPE_MAP: phf::Map<&'static str, ResourceType> = phf_map! {
    "Fire" => ResourceType::Fire,
    "Wood" => ResourceType::Wood,
    "Earth" => ResourceType::Earth,
    "Water" => ResourceType::Water,
    "Metal" => ResourceType::Metal,
    "Shard" => ResourceType::Shard,
    "Prismite" => ResourceType::Prismite,
    "Dormant" => ResourceType::Dormant,
};

/// a direct representation of the data structure encoded in the core_cards.json file.
/// Is used only to post-process into usable card prototype data.
#[derive(Serialize, Deserialize, Debug)]
struct RawCardData {
    name: String,

    #[serde(rename = "cost")]
    affinity: String,

    #[serde(rename = "total_cost")]
    cost: String,

    #[serde(rename = "type")]
    card_type: String,
    text: String,

    // these properties are not used for now

    //power: String,
    //toughness: String,
    //#[serde(rename = "image")]
    //image_name: String,
    //revision_date_time
    //details: String,
    //factions: Vec<String>,
    //rulings: Vec<String>,
    //complexity: String,
}

/// Parses the core_cards.json file created by C. Gannon into a
/// HashMap<CardPrototypeId, CardPrototype>, for use in the Algomancer Game Rules Engine
pub fn parse_json(raw_json: &str) -> Result<HashMap<CardPrototypeId, CardPrototype>, serde_json::Error> {
    let data: HashMap<String, Vec<RawCardData>> = serde_json::from_str(raw_json)?;

    let mut c = 0;
    let mapped: Vec<CardPrototype> = data.into_values().map(|mut d| {
        let d = d.remove(0);
        c = c + 1;

        CardPrototype {
            prototype_id: CardPrototypeId(c),
            name: d.name.clone(),
            text: d.text,
            costs: map_cost(&d.cost, &d.affinity),
            card_type: card_type_from_string(&d.card_type),
            std_name: std_name_from_string(&d.name).to_string(),
        }
    }).collect();

    let mut hashmap = HashMap::new();
    for d in mapped {
        if let CardType::Unit(_) = &d.card_type {
            hashmap.insert(d.prototype_id, d);
        } else {
            hashmap.insert(d.prototype_id, d);
        }
    }

    Ok(hashmap)
}

/// Maps a cost string & affinities string to a Cost Enum.
/// Handles X costs with threshold and the standard 'threshold + generic cost' strings.
fn map_cost(cost: &str, affinities: &str) -> Cost {
    if cost == "X" {
        Cost::X {
            threshold: parse_affinity_string(affinities),
        }
    } else {
        let cost_num = cost.parse().unwrap();
        Cost::Standard {
            threshold: parse_affinity_string(affinities),
            cost: cost_num,
        }
    }
}

/// Maps a card type string into the CardType enum
fn card_type_from_string(card_type: &str) -> CardType {
    if card_type.contains(SPELL_STR) {
        return if card_type.contains(TOKEN_STR) {
            CardType::SpellToken
        } else {
            CardType::Spell(timing_from_string(card_type))
        };
    } else if card_type.contains(META_RESOURCE_STR) {
        return CardType::Meta(meta_card_type_from_string(card_type));
    } else if card_type.contains(RESOURCE_STR) {
        return CardType::Resource(resource_type_from_string(card_type));
    }

    return if card_type.contains(TOKEN_STR) {
        CardType::UnitToken
    } else {
        CardType::Unit(timing_from_string(card_type))
    };
}

fn timing_from_string(card_type: &str) -> Timing {
    if card_type.contains(&HASTE_STR.to_string()) {
        Timing::Haste
    } else if card_type.contains(&VIRUS_STR.to_string()) {
        Timing::Virus
    } else if card_type.contains(&BATTLE_STR.to_string()) {
        Timing::Battle
    } else {
        Timing::Default
    }
}

fn meta_card_type_from_string(card_type: &str) -> MetaCardType {
    if card_type.contains(TRIGGER_STR) {
        MetaCardType::Trigger
    } else if card_type.contains(STOLEN_CARD_STR) {
        MetaCardType::StolenCard
    } else {
        panic!("not a valid meta card type {card_type}")
    }
}

fn resource_type_from_string(card_type: &str) -> ResourceType {
    for (str, fac) in RESOURCE_TYPE_MAP.into_iter() {
        if card_type.contains(str) {
            return *fac;
        }
    }

    panic!("not a valid resource type {card_type}")
}

/// The 'Standard Name' of a card prototype is a cleaner version of the normal card name, which is
/// used to relate the card json to an image file, or potentially whatever else needs a cleaner
/// string (no special chars, should work as a file name).
fn std_name_from_string(filename: &str) -> &str {
    fn remove_extension(filename: &str) -> &str {
        match filename.rfind('.') {
            Some(index) => &filename[..index],
            None => filename, // Return the original string if no dot is found
        }
    }

    let std_name = filename.trim();
    let std_name = remove_extension(std_name);

    std_name
}

/// Maps a character to the Faction enum.
fn faction_from_char(c: char) -> Option<Faction> {
    match c {
        'r' => Some(Faction::Fire),
        'e' => Some(Faction::Earth),
        'b' => Some(Faction::Water),
        'm' => Some(Faction::Metal),
        'g' => Some(Faction::Wood),
        _ => None
    }
}

/// Parse an affinity string  into a map of the faction and occurrence count
/// # Examples
/// 'rbb' -> (Fire, 1), (Water, 2)
fn parse_affinity_string(affinity: &str) -> Vec<Affinity> {
    let mut counts = HashMap::new();

    for ch in affinity.chars() {
        if let Some(faction) = faction_from_char(ch) {
            *counts.entry(faction).or_insert(0) += 1;
        } else {
            eprintln!("a faction does not exist for char '{ch}' in the affinity string '{affinity}'");
        }
    }

    counts.into_iter().map(|e| {
        Affinity {
            faction: e.0,
            quantity: e.1,
        }
    }).collect()
}


#[cfg(test)]
mod tests {
    use crate::CardPrototypeDatabase;

    #[test]
    fn test_load_from_raw_file() {
        let path = "../resources/core_cards.json";
        let db = CardPrototypeDatabase::from_path(path).unwrap();

        for d in db.prototypes.values() {
            println!("{}: {:?} | {} [{:?}] -- Cost: {:?}", d.prototype_id.0, d.card_type, d.name, d.std_name, d.costs);
        }
    }
}
