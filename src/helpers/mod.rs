pub fn round_point2(point: (f32, f32)) -> (usize, usize) {
    let x = point.0.round() as usize;
    let y = point.1.round() as usize;
    return (x, y);
}
