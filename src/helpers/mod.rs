pub fn round_point2(point: (f32, f32)) -> (usize, usize) {
    let x = point.0.round() as usize;
    let y = point.1.round() as usize;
    (x, y)
}

pub fn point2_f32(point: (usize, usize)) -> (f32, f32) {
    let x = point.0 as f32;
    let y = point.1 as f32;
    (x, y)
}
