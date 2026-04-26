#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct Skrunkle {
    pub palette: [String; 8],
    pub bg_color: String,
    pub strokes: Vec<Stroke>,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct Stroke {
    pub color: f64,
    pub shape: Vec<[f64; 3]>,
}
