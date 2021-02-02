

pub struct Complex {
    pub real : f32,
    pub imag : f32,
}

impl Complex {
    pub fn new(real : f32, imag : f32) -> Self {
        Self {
            real,
            imag,
        }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            real : self.real,
            imag : - self.imag,
        }
    }

    pub fn argument(&self) -> f32 {
        self.imag.atan2(self.real)
    }

    pub fn modulus(&self) -> f32 {
        (self.real*self.real + self.imag*self.imag).sqrt()
    }
}

pub struct EComplex {
    pub modulus : f32,
    pub argument : f32,
}

impl Complex {
    pub fn new(modulus : f32, argument : f32) -> Self {
        Self {
            modulus,
            argument,
        }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            modulus : self.modulus,
            argument : - self.argument,
        }
    }

    pub fn real(&self) -> f32 {
        self.modulus * self.argument.cos()
    }

    pub fn iamg(&self) -> f32 {
        self.modulus * self.argument.sin()
    }
}