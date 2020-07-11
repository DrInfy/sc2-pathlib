use std::cmp::{max, min};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rectangle {
    pub x: usize,
    pub y: usize,
    pub x_end: usize,
    pub y_end: usize,
}

impl Rectangle {
    pub fn init_from_center(center: (f64, f64), size: (usize, usize), width: usize, height: usize) -> Rectangle {
        let pos_x: usize = center.0 as usize;
        let pos_y: usize = center.1 as usize;

        let w: usize = size.0;
        let h: usize = size.1;
        let x: usize = max(0, (pos_x as f64 - (w as f64 / 2_f64)).ceil() as usize);
        let y: usize = max(0, (pos_y as f64 - (h as f64 / 2_f64)).ceil() as usize);
        let x_end: usize = min(width, w + x);
        let y_end: usize = min(height, h + y);

        Rectangle { x,
                    y,
                    x_end,
                    y_end }
    }

    pub fn init_from_center2(center: (usize, usize), size: (usize, usize), width: usize, height: usize) -> Rectangle {
        let pos_x: usize = center.0;
        let pos_y: usize = center.1;

        let w: usize = size.0;
        let h: usize = size.1;
        let x: usize = f64::max(0.0, (pos_x as f64 - (w as f64 / 2_f64)).ceil()) as usize;
        let y: usize = f64::max(0.0, (pos_y as f64 - (h as f64 / 2_f64)).ceil()) as usize;
        let x_end: usize = min(width, w + x);
        let y_end: usize = min(height, h + y);

        Rectangle { x,
                    y,
                    x_end,
                    y_end }
    }
}
