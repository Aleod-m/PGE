#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2D {
    pub x : f64,
    pub y : f64,
}

impl Vec2D {
    pub fn new(x : f64, y : f64) -> Self {
        Self {x : x, y : y}
    }
    pub fn sq_norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn norm(&self) -> f64 {
        self.sq_norm().sqrt()
    }

    pub fn normalize(&mut self){
        let k = 1.0_f64 / self.norm();
        self.x *= k;
        self.y *= k;
        
    }

    pub fn dot(&self, v2 : &Self) -> f64 {
        self.x * v2.x + self.y *v2.y
    }

    pub fn rot(&mut self, angle : &f64){
        self.x *= angle.cos();
        self.y *= angle.sin();
        
    }

    pub fn det(&self, v2 : &Self) -> f64 {
        self.x * v2.y - self.y *v2.x
    }

    pub fn angle(&self, v2 : &Self) -> f64 {
        self.dot(v2) / (self.norm() * v2.norm())
    }

    pub fn null() -> Self{
        Vec2D {
            x: 0_f64,
            y: 0_f64,
        }
    }
    pub fn up() -> Self {
        Self {
            x: 1_f64,
            y: 0_f64,
        }
    }
    pub fn down() -> Self {
        Self {
            x: -1_f64,
            y: 0_f64,
        }
    }
    pub fn right() -> Self {
        Self {
            x : 0_f64,
            y : -1_f64,
        }
    }
    pub fn left() -> Self {
        Self {
            x : 0_f64,
            y : 1_f64,
        }
    }

    pub fn to_vec(&self) -> Vec<f64> {
        vec![self.x, self.y]
    }
}


impl From<(f64, f64)> for Vec2D {
    fn from(other: (f64, f64)) -> Self {
        Vec2D::new(other.0, other.1)
    }
}




impl std::ops::Add for Vec2D {
    type Output = Vec2D;
    fn add(self, other : Self) -> Self {
        Self{
            x : self.x + other.x,
            y : self.y + other.y,
        }
    }
}

impl std::ops::Add<f64> for Vec2D {
    type Output = Vec2D;
    fn add(self, other : f64) -> Self {
        Self{
            x : self.x + other,
            y : self.y + other,
        }
    }
}

impl std::ops::AddAssign for Vec2D {
    fn add_assign(&mut self, other : Self) {
        *self = Self{
            x : self.x + other.x,
            y : self.y + other.y,
        };
    }
}
impl std::ops::Sub for Vec2D {
    type Output = Vec2D;
    fn sub(self, other : Self) -> Self {
        Self{
            x : self.x - other.x,
            y : self.y - other.y,
        }
    }
}
impl std::ops::Neg for Vec2D {
    type Output = Vec2D;
    fn neg(self) -> Self {
        Self{
            x : -self.x,
            y : -self.y,
        }
    }
}
impl std::ops::Mul<f64> for Vec2D {
    type Output = Vec2D;
    fn mul(self, other : f64) -> Self {
        Self{
            x : self.x * other,
            y : self.y * other,
        }
    }
}
impl std::ops::MulAssign<f64> for Vec2D {
    fn mul_assign(&mut self, other : f64) {
        *self = Self{
            x : self.x * other,
            y : self.y * other,
        };
    }
}

impl std::fmt::Display for Vec2D{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
