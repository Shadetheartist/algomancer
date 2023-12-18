use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

use algocore::{*};

#[derive(Serialize, Deserialize, Debug)]
struct CardDataRaw {
    name: String,
    power: String,
    toughness: String,
    #[serde(rename = "cost")]
    affinity: String,
    #[serde(rename = "total_cost")]
    cost: String,
    #[serde(rename = "type")]
    card_type: String,
    text: String,
    #[serde(rename = "image")]
    image_name: String,
    //revision_date_time
    details: String,
    factions: Vec<String>,
    rulings: Vec<String>,
    complexity: String,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy, Ord, PartialOrd)]
pub struct CardPrototypeId(pub usize);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CardPrototype {
    pub prototype_id: CardPrototypeId,
    pub name: String,
    pub text: String,
    pub costs: Cost,
    pub card_type: CardType,

    pub std_name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CardPrototypeDatabase {
    pub prototypes: HashMap<CardPrototypeId, CardPrototype>,
}


const HASTE_STR: &str = "{Haste}";
const VIRUS_STR: &str = "{Virus}";
const BATTLE_STR: &str = "{Battle}";


impl CardPrototypeDatabase {
    pub fn resource(&self, resource_type: ResourceType) -> &CardPrototype {
        self.prototypes.iter().find(|(_, c)| {
            c.card_type == CardType::Resource(resource_type)
        }).expect("a prototype for this resource").1
    }


    pub fn load_from_raw_json(raw_json: &str) -> Result<Self, serde_json::Error> {
        let data: HashMap<String, Vec<CardDataRaw>> = serde_json::from_str(raw_json)?;

        fn string_to_factions(affinity: &str) -> Vec<(Faction, u32)> {
            let mut counts = HashMap::new();

            for ch in affinity.chars() {
                let faction = match ch {
                    'r' => Faction::Fire,
                    'e' => Faction::Earth,
                    'w' => Faction::Water,
                    'm' => Faction::Metal,
                    'g' => Faction::Wood,
                    _ => continue, // Skip unrecognized characters
                };
                *counts.entry(faction).or_insert(0) += 1;
            }

            counts.into_iter().collect()
        }

        fn map_cost(cost: &str, affinities: &str) -> Cost {
            if cost == "X" {
                Cost::X {
                    faction_affinities: string_to_factions(affinities),
                }
            } else {
                let cost_num = cost.parse().unwrap();
                Cost::Standard {
                    faction_affinities: string_to_factions(affinities),
                    additional_cost: cost_num,
                }
            }
        }

        fn map_type(card_type: &str) -> CardType {
            fn timing(card_type: &str) -> Timing {
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

            fn meta_card_type(card_type: &str) -> MetaCardType {
                if card_type.contains("Trigger") {
                    MetaCardType::Trigger
                } else if card_type.contains("Stolen Card") {
                    MetaCardType::StolenCard
                } else {
                    panic!("not a valid meta card type {card_type}")
                }
            }

            fn resource_type(card_type: &str) -> ResourceType {
                if card_type.contains("Fire") {
                    ResourceType::Fire
                } else if card_type.contains("Wood") {
                    ResourceType::Wood
                } else if card_type.contains("Earth") {
                    ResourceType::Earth
                } else if card_type.contains("Water") {
                    ResourceType::Water
                } else if card_type.contains("Metal") {
                    ResourceType::Metal
                } else if card_type.contains("Shard") {
                    ResourceType::Shard
                } else if card_type.contains("Mana Converter") {
                    ResourceType::ManaConverter
                } else if card_type.contains("Prismite") {
                    ResourceType::Prismite
                } else if card_type.contains("Dormant") {
                    ResourceType::Dormant
                } else {
                    panic!("not a valid resource type {card_type}")
                }
            }

            if card_type.contains("Spell") {
                return CardType::Spell(timing(card_type));
            } else if card_type.contains("!Resource") {
                return CardType::Meta(meta_card_type(card_type));
            } else if card_type.contains("Resource") {
                return CardType::Resource(resource_type(card_type));
            }

            CardType::Unit(timing(card_type))
        }

        fn map_std_name(filename: &str) -> Box<str> {
            fn remove_extension(filename: &str) -> &str {
                match filename.rfind('.') {
                    Some(index) => &filename[..index],
                    None => filename, // Return the original string if no dot is found
                }
            }

            let std_name = filename.trim();
            let std_name = remove_extension(std_name);
            let std_name = std_name.replace('-', "_");

            std_name.into_boxed_str()
        }

        let mut c = 0;
        let mapped: Vec<CardPrototype> = data.into_values().map(|mut d| {
            let d = d.remove(0);
            c = c + 1;

            CardPrototype {
                prototype_id: CardPrototypeId(c),
                name: d.name,
                text: d.text,
                costs: map_cost(&d.cost, &d.affinity),
                card_type: map_type(&d.card_type),
                std_name: map_std_name(&d.image_name).to_string(),
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

        Ok(CardPrototypeDatabase {
            prototypes: hashmap
        })
    }

    pub fn load_from_raw_file(file_path: &str) -> Result<Self, DbError> {
        match fs::read_to_string(file_path) {
            Ok(file_content) => {
                match Self::load_from_raw_json(file_content.as_str()) {
                    Ok(db) => {
                        Ok(db)
                    }
                    Err(err) => {
                        Err(DbError::Serde(err))
                    }
                }
            }
            Err(err) => {
                Err(DbError::IO(err))
            }
        }

    }
}


#[cfg(test)]
mod tests {
    use crate::CardPrototypeDatabase;

    #[test]
    fn test_load_from_raw_file() {
        let path = "../resources/core_cards.json";
        let db = CardPrototypeDatabase::load_from_raw_file(path).unwrap();

        for d in db.prototypes.values() {
            println!("{}: {:?} | {} [{:?}] -- Cost: {:?}", d.prototype_id.0, d.card_type, d.name, d.std_name, d.costs);
        }
    }
}
