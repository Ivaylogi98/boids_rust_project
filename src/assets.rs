use ggez::audio;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::fmt::Debug;

pub struct Assets {
    pub bird: graphics::Image,
    pub obstacle: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let bird = graphics::Image::new(ctx, "./assets/bird.png")?;
        let obstacle = graphics::Image::new(ctx, "./assets/obstacle.png")?;
        Ok(Assets {
            bird,
            obstacle
        })
    }
}