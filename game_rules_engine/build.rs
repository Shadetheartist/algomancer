use database::CardPrototypeDatabase;

fn main() {

    // try parsing the card prototype database from the core_cards.json file as a build step,
    // so that if there is a parsing error, the compiler tells us rather than the runtime.
    // I consider the core_cards.json file a static resource.
    let core_cards_json_file = "../resources/core_cards.json";
    println!("cargo:rerun-if-changed={core_cards_json_file}");

    if let Err(err) = CardPrototypeDatabase::from_path(core_cards_json_file) {
        println!("cargo:warning=could not generate card db, error loading the db from file: {:?}", err);
    }

}