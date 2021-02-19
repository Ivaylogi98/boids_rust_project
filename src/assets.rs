use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Assets {
    pub bird: graphics::Image,
    pub obstacle: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let bird = graphics::Image::new(ctx, "/bird.png")?;
        let obstacle = graphics::Image::new(ctx, "/obstacle.png")?;
        Ok(Assets {
            bird,
            obstacle
        })
    }
}