#[derive(Debug)]
pub enum Flavour {
    AoE2DE,
}

impl Flavour {
    pub fn from_str(name: &str) -> Option<Flavour> {
        match name {
            "AoE2:DE" => Some(Flavour::AoE2DE),
            _ => None,
        }
    }
}