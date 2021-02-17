use core::time;

use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::MeshBuilder;
use ggez::mint::{Point2, Vector2};
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::assets::Assets;


#[derive(Debug, Copy, Clone)]
pub struct Bird {
    pub pos: Point2<f32>,
    pub vel: Vector2<f32>,
    pub is_alive: bool
}

impl Bird{
    pub const MAX_VELOCITY: f32 = 2 as f32;

    pub fn new(pos: Point2<f32>, vel: Vector2<f32>) -> Self {
        Bird{
            pos: pos,
            vel: vel,
            is_alive: true
        }
    }

    pub fn update(&mut self, acceleration: Vector2<f32>, screen_width: f32, screen_height: f32) {
        // update velocity
        self.vel.x += acceleration.x;
        self.vel.y += acceleration.y;
        self.limit_velocity(Bird::MAX_VELOCITY);

        // update position
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        if self.pos.x < 0.0 {
            self.pos.x += screen_width;
        }
        else if self.pos.x > screen_width {
            self.pos.x -= screen_width;
        }
        if self.pos.y < 0.0 {
            self.pos.y += screen_height;
        }
        else if self.pos.y > screen_height {
            self.pos.y -= screen_height;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        let drawparams = graphics::DrawParam::new()
                                .dest(self.pos)
                                .scale(Vector2{ x: 0.1, y: 0.1 })
                                .offset(Point2{ x: 0.47, y: 0.7 })
                                .rotation(-(self.vel.y / self.vel.x).atan() + 3.1415);
        graphics::draw(ctx, &assets.bird, drawparams)
    }

    fn limit_velocity(&mut self, max: f32) {
        let speed = (self.vel.x.powf(2.0) + self.vel.y.powf(2.0)).sqrt();
        if speed > max {
            self.vel.x *= max / speed;
            self.vel.y *= max / speed;
        }
    }

    pub fn view_distance_circle(&self, ctx: &mut Context, view_distance: f32) -> graphics::Mesh {
        MeshBuilder::new().circle(graphics::DrawMode::stroke(1.0), Point2{x: self.pos.x, y: self.pos.y}, view_distance, 1.0, (255, 0, 0).into()).build(ctx).unwrap()
    }
    pub fn center_point(&self, ctx: &mut Context) -> graphics::Mesh {
        MeshBuilder::new().circle(graphics::DrawMode::fill(), Point2{x: self.pos.x, y: self.pos.y}, 4.0, 1.0, (255, 0, 0).into()).build(ctx).unwrap()
    }
}

#[derive(Debug)]
pub struct Obstacle {
    pub pos: Point2<f32>,
    pub radius: f32
}

impl Obstacle{

    pub fn new(pos: Point2<f32>, radius: f32) -> Self {
        Obstacle{
            pos,
            radius
        }
    }
    pub fn update(&mut self) {
        todo!()        
    }
    pub fn draw(&mut self) {
        todo!()
    }
}