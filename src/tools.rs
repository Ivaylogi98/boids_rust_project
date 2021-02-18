use ggez::nalgebra::{ Vector2, Point2 };
use crate::entities::Bird;


pub struct Tools { }
impl Tools{ 
    pub fn normalize_vector( vec: &mut Vector2<f32> ) {
        let length = Self::vector_length(vec);
        vec.x /= length;
        vec.y /= length;
    }

    pub fn dist(b1: &Bird, b2: &Bird) -> f32 {
        ((b1.pos.x - b2.pos.x).powf(2.0) + (b1.pos.y - b2.pos.y).powf(2.0)).sqrt()
    }

    pub fn get_vec_from_to( p1: Point2<f32>, p2: Point2<f32> ) -> Vector2<f32> {
        Vector2::new(p1.x - p2.x, p1.y - p2.y)
    }

    pub fn limit_velocity(v: &mut Vector2<f32> , max: f32) {
        let speed = (v.x.powf(2.0) + v.y.powf(2.0)).sqrt();
        if speed > max {
            v.x *= max / speed;
            v.y *= max / speed;
        }
    }
    pub fn vector_length( vec: &Vector2<f32>) -> f32{
        (vec.x.powf(2.0) + vec.y.powf(2.0)).sqrt()
    }
}