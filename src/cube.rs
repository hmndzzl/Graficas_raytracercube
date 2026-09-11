use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::Vec3;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
    pub top_material: Option<Material>,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let mut tmin = (self.min.x - ray_origin.x) / ray_direction.x;
        let mut tmax = (self.max.x - ray_origin.x) / ray_direction.x;
        if tmin > tmax { std::mem::swap(&mut tmin, &mut tmax); }

        let mut tymin = (self.min.y - ray_origin.y) / ray_direction.y;
        let mut tymax = (self.max.y - ray_origin.y) / ray_direction.y;
        if tymin > tymax { std::mem::swap(&mut tymin, &mut tymax); }

        if tmin > tymax || tymin > tmax { return None; }
        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }

        let mut tzmin = (self.min.z - ray_origin.z) / ray_direction.z;
        let mut tzmax = (self.max.z - ray_origin.z) / ray_direction.z;
        if tzmin > tzmax { std::mem::swap(&mut tzmin, &mut tzmax); }

        if tmin > tzmax || tzmin > tmax { return None; }
        if tzmin > tmin { tmin = tzmin; }
        if tzmax < tmax { tmax = tzmax; }

        let distance = if tmin < 0.0 { tmax } else { tmin };
        if distance < 0.0 { return None; }

        let point = ray_origin + ray_direction * distance;
        
        let epsilon = 1e-4;
        let u;
        let v;
        let mut hit_top = false;
        
        let normal = if (point.x - self.max.x).abs() < epsilon {
            u = (point.z - self.min.z) / (self.max.z - self.min.z);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
            Vec3::new(1.0, 0.0, 0.0)
        } else if (point.x - self.min.x).abs() < epsilon {
            u = (self.max.z - point.z) / (self.max.z - self.min.z);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (point.y - self.max.y).abs() < epsilon {
            u = (point.x - self.min.x) / (self.max.x - self.min.x);
            v = (self.max.z - point.z) / (self.max.z - self.min.z);
            hit_top = true;
            Vec3::new(0.0, 1.0, 0.0)
        } else if (point.y - self.min.y).abs() < epsilon {
            u = (point.x - self.min.x) / (self.max.x - self.min.x);
            v = (point.z - self.min.z) / (self.max.z - self.min.z);
            Vec3::new(0.0, -1.0, 0.0)
        } else if (point.z - self.max.z).abs() < epsilon {
            u = (self.max.x - point.x) / (self.max.x - self.min.x);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            u = (point.x - self.min.x) / (self.max.x - self.min.x);
            v = (point.y - self.min.y) / (self.max.y - self.min.y);
            Vec3::new(0.0, 0.0, -1.0)
        };

        let selected_material = if hit_top {
            self.top_material.as_ref().unwrap_or(&self.material).clone()
        } else {
            self.material.clone()
        };

        Some(Intersect {
            point,
            normal,
            distance,
            u,
            v,
            material: selected_material,
        })
    }
}
