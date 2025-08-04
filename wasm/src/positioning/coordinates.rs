#[derive(Clone)]
#[derive(Copy)]
pub struct Cartesian {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Cartesian {
    pub fn to_cylindrical(&self) -> Cylindrical {
        let r = (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
        let theta = (self.y / self.x).atan();
        let z = self.z;

        Cylindrical { r, theta, z }
    }

    pub fn to_spherical(&self) -> Spherical {
        let r = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
        let theta = (self.y / self.x).atan();
        let phi = (self.z / r).acos();

        Spherical { r, theta, phi }
    }
}

// American physicist convention is to use "rho" instead of "r".
// I have chosen the international ISO 80000-2:2019 representation.
#[derive(Clone)]
#[derive(Copy)]
pub struct Spherical {
    pub r: f32, // rho
    pub theta: f32,
    pub phi: f32,
}

impl Spherical {
    pub fn to_cartesian(&self) -> Cartesian {
        let x = self.r * self.phi.sin() * self.theta.cos();
        let y = self.r * self.phi.sin() * self.theta.sin();
        let z = self.r * self.phi.cos();

        Cartesian { x, y, z }
    }
}

// There is some deviation in naming conventions.
// Physicists use "rho" and "phi" instead of "r" and "theta".
// I have chosen to use the mathematical conventions because it makes it easier
// to check my work against mathematical equations.
#[derive(Clone)]
#[derive(Copy)]
pub struct Cylindrical {
    pub r: f32, // rho
    pub theta: f32, // phi
    pub z: f32,
}

impl Cylindrical {
    pub fn to_cartesian(&self) -> Cartesian {
        let x = self.r * self.theta.cos();
        let y = self.r * self.theta.sin();
        let z = self.z;

        Cartesian { x, y, z }
    }
}
