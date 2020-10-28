use crate::math::quat::Quat;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3D {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64 , z: f64) -> Self {
        Self {
            x : x,
            y : y,
            z : z,
        }
    }
    pub fn sq_norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z* self.z
    }

    pub fn norm(&self) -> f64 {
        self.sq_norm().sqrt()
    }

    pub fn normalize(&mut self) {
        let k = 1.0_f64 / self.norm();
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    pub fn normalized(&self) -> Self{
        let k = 1.0_f64 / self.norm();
        Self {
            x:self.x * k,
            y:self.y * k,
            z:self.z * k,
        }
    }

    pub fn dot(&self, v2 : &Self) -> f64 {
        self.x * v2.x + self.y *v2.y + self.z * v2.z
    }

    pub fn cross(&self, v2 : &Self) -> Self {
        Self{
            x : self.y*v2.z-self.z*v2.y,
            y : self.x*v2.z-self.z*v2.x,
            z : self.x*v2.y-self.y*v2.x,
        }
    }

    pub fn angle(&self, v2 : &Self) -> f64 {
        let cnorm = self.cross(v2).norm();
        cnorm / (self.norm() * v2.norm())
    }

    pub fn null() -> Self{
        Vec3D {
            x: 0_f64,
            y: 0_f64,
            z: 0_f64,
        }
    }
    pub fn up() -> Self {
        Self{
            x:0_f64,
            y:0_f64,
            z:1_f64,
        }
    }
    pub fn down() -> Self {
        Self{
            x:0_f64,
            y:0_f64,
            z:-1_f64,
        }
    }
    pub fn forward() -> Self {
        Self{
            x:1_f64,
            y:0_f64,
            z:0_f64,
        }
    }
    pub fn back() -> Self {
        Self{
            x:-1_f64,
            y:0_f64,
            z:0_f64,
        }
    }
    pub fn right() -> Self {
        Self{
            x:0_f64,
            y:-1_f64,
            z:0_f64,
        }
    }
    pub fn left() -> Self {
        Self{
            x:0_f64,
            y:1_f64,
            z:0_f64,
        }
    }

    pub fn rotate(&mut self, angle : f64, axis : &Vec3D) -> Vec3D {
        let qvec = Quat {
            s: 0_f64,
            v: *self
        };
        let qrot = Quat {
            s : (angle*0.5).cos(),
            v : (angle*0.5).sin() * axis.normalized(),
        };

        return (qrot * qvec * qrot.inv()).v
    }

    pub fn to_vec(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z]

    }
}

impl From<(f64, f64, f64)> for Vec3D {
    fn from(other: (f64, f64, f64)) -> Self {
        Vec3D::new(other.0, other.1, other.2)
    }
}

impl std::ops::Add for Vec3D {
    type Output = Vec3D;
    fn add(self, other : Self) -> Self {
        Self{
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z,
        }
    }
}
impl std::ops::AddAssign for Vec3D {
    fn add_assign(&mut self, other : Self) {
        *self = Self{
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z,
        };
    }
}
impl std::ops::Sub for Vec3D {
    type Output = Vec3D;
    fn sub(self, other : Self) -> Self {
        Self{
            x : self.x - other.x,
            y : self.y - other.y,
            z : self.z - other.z,
        }
    }
}
impl std::ops::Neg for Vec3D {
    type Output = Vec3D;
    fn neg(self) -> Self {
        Self{
            x : -self.x,
            y : -self.y,
            z : -self.z,
        }
    }
}
impl std::ops::Mul<f64> for Vec3D {
    type Output = Vec3D;
    fn mul(self, other : f64) -> Self {
        Self{
            x : self.x * other,
            y : self.y * other,
            z : self.z * other,
        }
    }
}
impl std::ops::MulAssign<f64> for Vec3D {
    fn mul_assign(&mut self, other : f64) {
        *self = Self{
            x : self.x * other,
            y : self.y * other,
            z : self.z * other,
        };
    }
}
impl std::ops::Mul<Vec3D> for f64 {
    type Output = Vec3D;
    fn mul(self, other : Vec3D) -> Vec3D {
        Vec3D {
            x : self * other.x,
            y : self * other.y,
            z : self * other.z,
        }
    }
}
impl std::fmt::Display for Vec3D{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
