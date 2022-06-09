use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

// constructors & setters
impl Ray {
    pub fn new() -> Ray {
        Ray {
            orig: Point3::new(),
            dir: Vec3::new(),
        }
    }

    pub fn set_fields(self, o: Point3, d: Vec3) -> Ray {
        Ray { orig: o, dir: d }
    }
}

// member functions
impl Ray {
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
