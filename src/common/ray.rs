#[derive(Debug, Clone)]
pub struct Ray {
    origin: nalgebra_glm::DVec3,
    direction: nalgebra_glm::DVec3
}

impl Ray {
    pub fn new(origin: nalgebra_glm::DVec3, direction: nalgebra_glm::DVec3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn origin(&self) -> &nalgebra_glm::DVec3 {
        &self.origin
    }

    pub fn direction(&self) -> &nalgebra_glm::DVec3 {
        &self.direction
    }
}