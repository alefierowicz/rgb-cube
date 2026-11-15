pub type Vec3 = [f32; 3];

// https://msl.cs.uiuc.edu/planning/node102.html, fig. 3.40 and 3.41
pub type Mat3 = [[f32; 3]; 3];

#[rustfmt::skip]
pub fn ry(angle: f32) -> Mat3 {
    let (s, c) = angle.sin_cos();
    [
        [ c , 0.0,  s ],
        [0.0, 1.0, 0.0],
        [-s , 0.0,  c ]
    ]
}
#[rustfmt::skip]
pub fn rx(angle: f32) -> Mat3 {
    let (s, c) = angle.sin_cos();
    [
        [1.0, 0.0, 0.0],
        [0.0,  c , -s ],
        [0.0,  s ,  c ]
    ]
}

// literally just matrix multiplication
pub fn mul(m: Mat3, v: Vec3) -> Vec3 {
    m.map(|row| row[0] * v[0] + row[1] * v[1] + row[2] * v[2])
}
