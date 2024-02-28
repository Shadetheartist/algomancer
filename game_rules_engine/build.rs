use database::CardPrototypeDatabase;

fn main() {
    let core_cards_json_file = "../resources/core_cards.json";
    println!("cargo:rerun-if-changed={core_cards_json_file}");

    if let Err(err) = CardPrototypeDatabase::from_path(core_cards_json_file) {
        println!("cargo:warning=could not generate card db, error loading the db from file: {:?}", err);
        //panic!("could not generate card db, error loading the db from file: {}", err)
    }

}