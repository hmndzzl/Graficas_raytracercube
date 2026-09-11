use crate::color::Color;
use crate::texture::Texture;
use std::sync::Arc;
use nalgebra_glm::Vec3;

#[derive(Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 3],
    pub texture: Option<Arc<Texture>>,
}

impl Material {
    pub fn new(diffuse: Color, specular: f32, albedo: [f32; 3]) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            texture: None,
        }
    }

    pub fn with_texture(mut self, texture: Arc<Texture>) -> Self {
        self.texture = Some(texture);
        self
    }
}

#[derive(Clone)]
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub u: f32,
    pub v: f32,
    pub material: Material,
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}
