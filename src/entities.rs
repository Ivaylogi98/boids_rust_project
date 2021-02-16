use core::time;

use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::MeshBuilder;
use ggez::mint::{Point2, Vector2};
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::assets::Assets;


#[derive(Debug)]
pub struct Bird {
    pub pos: Point2<f32>,
    pub orient: f32,
    time_until_orient_update: f32
}

impl Bird{
    pub const SPEED: f32 = 1.5 as f32;
    pub const VIEW_DISTANCE: f32 = 500 as f32;

    pub fn new(pos: Point2<f32>, orient: f32) -> Self {
        Bird{
            pos: pos,
            orient: orient,
            time_until_orient_update: 0.1
        }
    }

    pub fn update(&mut self, seconds: f32, rng: &mut ThreadRng) {
        // update position
        let x_offset = self.orient.sin() * Self::SPEED;
        let y_offset = self.orient.cos() * Self::SPEED;
        self.pos.x += x_offset;
        self.pos.y += y_offset;

        // update orientation
        self.time_until_orient_update -= seconds;
        let mut orientation_rand_offset = 0 as f32;
        if self.time_until_orient_update <= 0 as f32 {
            self.time_until_orient_update = 0.1 as f32;
            orientation_rand_offset = rng.gen_range(-0.261799..0.261799);
        }
        
        self.orient += orientation_rand_offset;
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        graphics::draw(ctx, &assets.bird, graphics::DrawParam {
            dest: self.pos,
            scale: Vector2 { x: 0.1, y: 0.1 },
            offset: Point2 { x: 0.47, y: 0.7 },
            rotation: -self.orient + 3.1415,
            .. Default::default()
        })?;
        let mesh  = MeshBuilder::new().circle(graphics::DrawMode::fill(), Point2{x: 0.0, y: 0.0}, 4.0, 1.0, (255, 0, 0).into()).build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam {
            dest: self.pos,
            .. Default::default()
        })?;
        Ok(())
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