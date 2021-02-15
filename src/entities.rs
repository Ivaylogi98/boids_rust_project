use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::mint::{Point2, Vector2};
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::assets::Assets;


#[derive(Debug)]
pub struct Bird {
    pub pos: Point2<f32>,
    pub orient: f32
}

impl Bird{
    pub const SPEED: f32 = 100 as f32;
    pub const VIEW_DISTANCE: f32 = 500 as f32;

    pub fn new(pos: Point2<f32>, orient: f32) -> Self {
        Bird{
            pos: pos,
            orient: orient
        }
    }

    pub fn update(&mut self, seconds: f32, mut rng: ThreadRng) {
        // update position
        let x_offset = self.orient.sin() * Self::SPEED;
        let y_offset = self.orient.cos() * Self::SPEED;
        self.pos.x += x_offset;
        self.pos.y += y_offset;

        // update orientation
        let rand = rng.gen_range(0 .. 10);
        self.orient += rand as f32;
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        graphics::draw(ctx, &assets.bird, graphics::DrawParam {
            dest: self.pos,
            scale: Vector2 { x: 1.0, y: 1.0 },
            offset: Point2 { x: 1.0, y: 1.0 },
            rotation: self.orient,
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