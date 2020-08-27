const PI2: f32 = std::f32::consts::PI * 2.0;

#[inline]
pub fn angle_between(first: (usize, usize), other: (usize, usize)) -> f32 {
    point_angle((other.0 as f32 - first.0 as f32, other.1 as f32 - first.1 as f32))
}

#[inline]
pub fn angle_between_f32(first: (f32, f32), other: (f32, f32)) -> f32 {
    point_angle((other.0 - first.0, other.1 - first.1))
}

// (x,y) = (1,0) => -pi /2
// (x,y) = (0,-1) => 0
// (x,y) = (0,1) => pi
// (x,y) = (-1,0) =>  pi / 2
#[inline]
pub fn point_angle(point: (f32, f32)) -> f32 {
    let mut angle: f32;
    if point.1 == 0.0 {
        if point.0 > 0.0 {
            angle = std::f32::consts::FRAC_PI_2
        } else if point.1 < 0.0 {
            angle = -std::f32::consts::FRAC_PI_2
        } else {
            angle = 0.0f32
        }
    } else {
        angle = (point.0 as f32 / point.1 as f32).atan();
        if point.1 >= 0.0 {
            angle += std::f32::consts::PI
        }
    }

    angle
}

pub fn wrap_angle(angle: f32) -> f32 {
    let mut angle = angle % PI2;

    if angle < -std::f32::consts::PI {
        angle += PI2
    } else if angle > std::f32::consts::PI {
        angle -= PI2
    }
    angle
}

pub fn angle_distance(angle1: f32, angle2: f32) -> f32 {
    let angle1 = wrap_angle(angle1);
    let angle2 = wrap_angle(angle2);
    let mut d = f32::abs(angle2 - angle1);

    if d <= std::f32::consts::PI {
        // Do nothing
    } else if angle1 < angle2 {
        d = f32::abs(angle2 - (angle1 + std::f32::consts::FRAC_PI_2))
    } else {
        d = f32::abs(angle2 + std::f32::consts::FRAC_PI_2 - angle1)
    }
    d
}
