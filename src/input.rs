use serde_derive::Deserialize;
use std::error::Error;
use std::io::Read;
use std::{fs, io::BufReader};

#[derive(Deserialize)]
pub struct SteelColumnDrawing {
    pub h_section: HSection,
    pub base_plate: BasePlate,
    pub anchor_bolt: AnchorBolt,
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
}

#[derive(Deserialize)]
pub struct BasePlate {
    pub lx: f64,
    pub ly: f64,
    pub t: f64,
}

#[derive(Deserialize)]
pub struct AnchorBolt {
    pub d: f64,
    pub l: f64,
    pub nx: u32,
    pub ny: u32,
    pub jx: f64,
    pub jy: f64,
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

fn read_file(path: &str) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(BufReader::new)
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

pub fn read_input(file_path: &str) -> Result<SteelColumnDrawing, Box<dyn Error>> {
    let s = read_file(file_path).expect("failed to read file");

    let toml: Result<SteelColumnDrawing, toml::de::Error> = toml::from_str(&s);

    let toml = toml.expect("failed to parse toml");

    Ok(toml)
}
