use std::f32;
use ultraviolet::*;

#[derive(Clone, Debug)]
pub struct AABB {
    pub p_min: Vec3,
    pub p_max: Vec3,
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            p_min: Vec3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX),
            p_max: Vec3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN),
        }
    }
}

impl AABB {
    pub fn is_valid(&self) -> bool {
        self.p_max.x >= self.p_min.x && self.p_max.y >= self.p_min.y && self.p_max.z >= self.p_min.z
    }

    pub fn union_aabb(&self, b: &AABB) -> AABB {
        AABB {
            p_min: self.p_min.min_by_component(b.p_min),
            p_max: self.p_max.max_by_component(b.p_max),
        }
    }

    pub fn union_vec(&self, v: &Vec3) -> AABB {
        AABB {
            p_min: self.p_min.min_by_component(*v),
            p_max: self.p_max.max_by_component(*v),
        }
    }

    #[inline]
    pub fn size(&self) -> Vec3 {
        self.p_max - self.p_min
    }

    #[inline]
    pub fn center(&self) -> Vec3 {
        self.size() * 0.5 + self.p_min
    }
}
