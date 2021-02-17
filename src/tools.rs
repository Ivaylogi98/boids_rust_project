use ggez::mint::{ Vector2, Point2 };
use crate::entities::Bird;


pub struct Tools { }
impl Tools{ 
    pub fn normalize_vector( vec: &mut Vector2<f32> ) {
        let length = Self::vector_length(vec);
        vec.x /= length;
        vec.y /= length;
    }

    pub fn distance(b1: &Bird, b2: &Bird) -> f32 {
        ((b1.pos.x - b2.pos.x).powf(2.0) + (b1.pos.y - b2.pos.y).powf(2.0)).sqrt()
    }

    pub fn get_vec_from_to( p1: Point2<f32>, p2: Point2<f32> ) -> Vector2<f32> {
        Vector2{ x: p1.x - p2.x, y: p1.y - p2.y }
    }

    pub fn vec_op( vec1: &mut Vector2<f32>, vec2: &Vector2<f32>, op: fn(f32, f32) -> f32) {
        vec1.x = op(vec1.x, vec2.x);
        vec1.y = op(vec1.y, vec2.y);
    }

    pub fn vec_scalar_op( v: &mut Vector2<f32>, scalar: f32, op: fn(f32, f32) -> f32){
        v.x = op(v.x, scalar);
        v.y = op(v.y, scalar);
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

    pub fn point_op( vec1: &mut Point2<f32>, vec2: &Point2<f32>, op: fn(f32, f32) -> f32) {
        vec1.x = op(vec1.x, vec2.x);
        vec1.y = op(vec1.y, vec2.y);
    }

    pub fn point_scalar_op( v: &mut Point2<f32>, scalar: f32, op: fn(f32, f32) -> f32){
        v.x = op(v.x, scalar);
        v.y = op(v.y, scalar);
    }
}