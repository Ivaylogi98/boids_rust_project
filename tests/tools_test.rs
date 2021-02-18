use boids::tools::Tools;
use ggez::nalgebra::{ Point2, Vector2 };

#[test]
fn test_normalization() {
    let mut vector: Vector2<f32> = Vector2::new(10.0, 0.0);
    let vector_norm: Vector2<f32> = Vector2::new(1.0, 0.0);
    Tools::normalize_vector(&mut vector);

    assert_eq!(vector, vector_norm);

    
    let mut vector: Vector2<f32> = Vector2::new(10.0, 10.0);
    let vector_norm: Vector2<f32> = Vector2::new(0.7071067811865475, 0.7071067811865475);
    Tools::normalize_vector(&mut vector);

    assert_eq!(vector, vector_norm);
}

#[test]
fn test_get_vec_from_to() {
    let point1: Point2<f32> = Point2::new(2.0,4.0);
    let point2: Point2<f32> = Point2::new(1.0,2.0);
    let vector1_2: Vector2<f32> = Tools::get_vec_from_to(point1, point2);
    let vector1_2_wanted: Vector2<f32> = Vector2::new(1.0, 2.0);

    assert_eq!(vector1_2, vector1_2_wanted);


    let vector2_1: Vector2<f32> = Tools::get_vec_from_to(point2, point1);
    let vector2_1_wanted: Vector2<f32> = Vector2::new(-1.0, -2.0);

    assert_eq!(vector2_1, vector2_1_wanted);
}

#[test]
fn test_limit_vector() {
    let mut vector: Vector2<f32> = Vector2::new(10.0, 0.0);
    Tools::limit_vector(&mut vector, 10.0);
    let limited_vector_wanted: Vector2<f32> = Vector2::new(10.0, 0.0);
    
    assert_eq!(vector, limited_vector_wanted);


    let mut vector: Vector2<f32> = Vector2::new(10.0, 0.0);
    Tools::limit_vector(&mut vector, 5.0);
    let limited_vector_wanted: Vector2<f32> = Vector2::new(5.0, 0.0);
    
    assert_eq!(vector, limited_vector_wanted);


    let mut vector: Vector2<f32> = Vector2::new(5.0, 5.0);
    Tools::limit_vector(&mut vector, 5.0);
    let limited_vector_wanted: Vector2<f32> = Vector2::new(3.5355339059327375, 3.5355339059327375);
    
    assert_eq!(vector, limited_vector_wanted);
}

#[test]
fn test_vector_length() {
    let mut vector: Vector2<f32> = Vector2::new(10.0, 0.0);
    let len: f32 = Tools::vector_length(&mut vector);
    
    assert_eq!(len, 10.0);


    let mut vector: Vector2<f32> = Vector2::new(10.0, 10.0);
    let len: f32 = Tools::vector_length(&mut vector);
    
    assert_eq!(len, 14.142135623730951);


    let mut vector: Vector2<f32> = Vector2::new(-5.0, 5.0);
    let len: f32 = Tools::vector_length(&mut vector);
    
    assert_eq!(len, 7.0710678118654755);
}