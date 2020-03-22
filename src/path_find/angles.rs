const PI2: f64 = std::f64::consts::PI * 2.0;

#[inline]
pub fn angle_between(first: (usize, usize), other: (usize, usize)) -> f64 {
    point_angle((other.0 as f64 - first.0 as f64, other.1 as f64 - first.1 as f64))
}

#[inline]
pub fn angle_between_f64(first: (f64, f64), other: (f64, f64)) -> f64 {
    point_angle((other.0 - first.0, other.1 - first.1))
}

// (x,y) = (1,0) => -pi /2
// (x,y) = (0,-1) => 0
// (x,y) = (0,1) => pi
// (x,y) = (-1,0) =>  pi / 2
#[inline]
pub fn point_angle(point: (f64, f64)) -> f64 {
    let mut angle: f64;
    if point.1 == 0.0 {
        if point.0 > 0.0 {
            angle = std::f64::consts::FRAC_PI_2
        } else if point.1 < 0.0 {
            angle = -std::f64::consts::FRAC_PI_2
        } else {
            angle = 0.0f64
        }
    } else {
        angle = (point.0 as f64 / point.1 as f64).atan();
        if point.1 >= 0.0 {
            angle += std::f64::consts::PI
        }
    }

    angle
}

pub fn wrap_angle(angle: f64) -> f64 {
    let mut angle = angle % PI2;

    if angle < -std::f64::consts::PI {
        angle += PI2
    } else if angle > std::f64::consts::PI {
        angle -= PI2
    }
    angle
}

pub fn angle_distance(angle1: f64, angle2: f64) -> f64 {
    let angle1 = wrap_angle(angle1);
    let angle2 = wrap_angle(angle2);
    let mut d = f64::abs(angle2 - angle1);

    if d <= std::f64::consts::PI {
        // Do nothing
    } else if angle1 < angle2 {
        d = f64::abs(angle2 - (angle1 + std::f64::consts::FRAC_PI_2))
    } else {
        d = f64::abs(angle2 + std::f64::consts::FRAC_PI_2 - angle1)
    }
    d
}
