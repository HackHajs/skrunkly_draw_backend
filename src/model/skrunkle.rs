#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct Skrunkle {
    palette: [String; 8],
    bg_color: String,
    strokes: Vec<Stroke>,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
struct Stroke {
    color: f64,
    shape: Vec<[f64; 3]>,
}
