use anyhow::Result;
use serde_derive::Deserialize;
use std::io::Read;
use std::{fs, io::BufReader};

#[derive(Deserialize)]
pub struct SteelColumnDrawing {
    pub column_name: String,
    pub h_section: HSection,
    pub base_plate: BasePlate,
    pub anchor_bolt: AnchorBolt,
    pub anchor_plate: AnchorPlate,
    #[serde(default)]
    pub layer_name: LayerName,
}

#[derive(Deserialize)]
pub struct HSection {
    pub h: f64,
    pub b: f64,
    pub tw: f64,
    pub tf: f64,
    #[serde(default)]
    pub r: f64,
    pub material: String,
}

#[derive(Deserialize)]
pub struct BasePlate {
    pub lx: f64,
    pub ly: f64,
    pub t: f64,
    pub material: String,
}

#[derive(Deserialize)]
pub struct AnchorBolt {
    pub d: f64,
    pub l: f64,
    pub nx: u32,
    pub ny: u32,
    pub jx: f64,
    pub jy: f64,
    pub material: String,
    pub note: String,
}

#[derive(Deserialize)]
pub struct AnchorPlate {
    pub t: f64,
    pub d: f64,
    pub material: String,
}

#[derive(Deserialize, Clone)]
pub struct LayerName {
    pub s_column: String,
    pub bolt: String,
    pub plate: String,
}

impl Default for LayerName {
    fn default() -> Self {
        Self {
            s_column: "S柱".to_string(),
            bolt: "Sボルト".to_string(),
            plate: "Sプレート".to_string(),
        }
    }
}

fn read_file(path: &str) -> Result<String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path).map(BufReader::new)?;

    fr.read_to_string(&mut file_content)?;

    Ok(file_content)
}

pub fn read_input(file_path: &str) -> Result<SteelColumnDrawing> {
    let s = read_file(file_path).expect("failed to read file");

    let toml: Result<SteelColumnDrawing, toml::de::Error> = toml::from_str(&s);

    let toml = toml.expect("failed to parse toml");

    Ok(toml)
}
