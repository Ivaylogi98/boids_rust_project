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
    pub orient: f32,
    pub is_alive: bool
}

impl Bird{
    pub const SPEED: f32 = 1.5 as f32;

    pub fn new(pos: Point2<f32>, orient: f32) -> Self {
        Bird{
            pos: pos,
            orient: orient,
            is_alive: true
        }
    }

    pub fn update(&mut self, orientation_update: f32) {
        // update position
        let x_offset = self.orient.sin() * Self::SPEED;
        let y_offset = self.orient.cos() * Self::SPEED;
        self.pos.x += x_offset;
        self.pos.y += y_offset;

        // update orientation
        self.orient += orientation_update;
        if self.orient <= 0.0 {
            self.orient += 6.283;
        }
        else if self.orient >= 6.283 {
            self.orient -= 6.283;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        graphics::draw(ctx, &assets.bird, graphics::DrawParam {
            dest: self.pos,
            scale: Vector2 { x: 0.1, y: 0.1 },
            offset: Point2 { x: 0.47, y: 0.7 },
            rotation: -self.orient + 3.1415,
            .. Default::default()
        })?;
        Ok(())
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