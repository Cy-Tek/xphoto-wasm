use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
pub enum FilterType {
    Oceanic,
    Islands,
    Marine,
    SeaGreen,
    FlagBlue,
    Liquid,
    Diamante,
    Radio,
    Twenties,
    RoseTint,
    Mauve,
    BlueChrome,
    Vintage,
    Perfume,
    Serenity
}

 /// Returns the stringified name of the given filter
 #[wasm_bindgen]
 pub fn get_filter_name(filter: FilterType) -> String {
    match filter {
        FilterType::Oceanic => "oceanic",
        FilterType::Islands => "islands",
        FilterType::Marine => "marine",
        FilterType::SeaGreen => "seagreen",
        FilterType::FlagBlue => "flagblue",
        FilterType::Liquid => "liquid",
        FilterType::Diamante => "diamante",
        FilterType::Radio => "radio",
        FilterType::Twenties => "twenties",
        FilterType::RoseTint => "rosetint",
        FilterType::Mauve => "mauve",
        FilterType::BlueChrome => "bluechrome",
        FilterType::Vintage => "vintage",
        FilterType::Perfume => "perfume",
        FilterType::Serenity => "serenity",
    }.to_string()
}