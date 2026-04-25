#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Skrunkle {
    palette: [String; 8],
    bg_color: String,
    strokes: Vec<Stroke>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Stroke {
    color: f64,
    shape: Vec<[f64; 3]>,
}
