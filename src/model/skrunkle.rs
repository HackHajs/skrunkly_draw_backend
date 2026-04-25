#[derive(serde::Serialize, serde::Deserialize)]
pub struct Skrunkle {
    palette: [String; 8],
    bg_color: String,
    strokes: Vec<Stroke>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Stroke {
    color: f64,
    shape: Vec<[f64; 3]>,
}
