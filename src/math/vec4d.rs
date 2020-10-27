#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec4D {
    pub x : f64,
    pub y : f64,
    pub z : f64,
    pub w : f64,
}

impl Vec4D {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            x : x,
            y : y,
            z : z,
            w : w,
        }
    }
    
    pub fn sq_norm(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }

    pub fn norm(&self) -> f64 {
        self.sq_norm().sqrt()
    }

    pub fn normalize(&mut self) {
        let k = 1.0_f64 / self.norm();
        self.x *= k;
        self.y *= k;
        self.z *= k;
        self.w *= k;
    }

    pub fn normalized(&self) -> Self {
        let k = 1.0_f64 / self.norm();
        Self {
            x:self.x * k,
            y:self.y * k,
            z:self.z * k,
            w:self.w * k,
        }
    }

    pub fn dot(&self, v2 : &Self) -> f64 {
        self.x*v2.x + self.y*v2.y + self.z*v2.z + self.w*v2.w
    }

    pub fn null() -> Self {
        Vec4D {
            x: 0_f64,
            y: 0_f64,
            z: 0_f64,
            w: 0_f64,
        }
    }
}

impl From<(f64, f64, f64, f64)> for Vec4D {
    fn from(other: (f64, f64, f64, f64)) -> Self {
        Vec4D::new(other.0, other.1, other.2, other.3)
    }
}

impl Into<Vec<f64>> for Vec4D {
    fn into(self) -> Vec<f64> {
        vec![self.x, self.y, self.z, self.w]
    }
}

impl std::ops::Add for Vec4D {
    type Output = Vec4D;
    fn add(self, other : Self) -> Self {
        Self{
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z,
            w : self.w + other.w,
        }
    }
}
impl std::ops::AddAssign for Vec4D {
    fn add_assign(&mut self, other : Self) {
        *self = Self{
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z,
            w : self.w + other.w,
        };
    }
}
impl std::ops::Sub for Vec4D {
    type Output = Vec4D;
    fn sub(self, other : Self) -> Self {
        Self{
            x : self.x - other.x,
            y : self.y - other.y,
            z : self.z - other.z,
            w : self.w - other.z,
        }
    }
}
impl std::ops::Neg for Vec4D {
    type Output = Vec4D;
    fn neg(self) -> Self {
        Self{
            x : -self.x,
            y : -self.y,
            z : -self.z,
            w : -self.w,
        }
    }
}
impl std::ops::Mul<f64> for Vec4D {
    type Output = Vec4D;
    fn mul(self, other : f64) -> Self {
        Self{
            x : self.x * other,
            y : self.y * other,
            z : self.z * other,
            w : self.w * other,
        }
    }
}
impl std::ops::MulAssign<f64> for Vec4D {
    fn mul_assign(&mut self, other : f64) {
        *self = Self{
            x : self.x * other,
            y : self.y * other,
            z : self.z * other,
            w : self.w * other,
        };
    }
}
impl std::ops::Mul<Vec4D> for f64 {
    type Output = Vec4D;
    fn mul(self, other : Vec4D) -> Vec4D {
        Vec4D {
            x : self * other.x,
            y : self * other.y,
            z : self * other.z,
            w : self * other.w,
        }
    }
}
impl std::fmt::Display for Vec4D{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{},{})", self.x, self.y, self.z, self.w)
    }
}
