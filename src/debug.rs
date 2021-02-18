  
use ggez::{Context, GameResult};
use ggez::graphics;

pub fn is_active() -> bool {
    std::env::var("DEBUG").is_ok()
}

pub fn draw_debug_info(
            alignment_view_distance_circle: graphics::Mesh,
            separation_view_distance_circle: graphics::Mesh,
            center_point: graphics::Mesh,
            alignment_vector: graphics::Mesh,
            separation_vector: graphics::Mesh,
            cohesion_vector: graphics::Mesh,
            ctx: &mut Context
        ) -> GameResult<()> {
    graphics::draw(ctx, &alignment_view_distance_circle, graphics::DrawParam::default())?;
    graphics::draw(ctx, &separation_view_distance_circle, graphics::DrawParam::default())?;
    graphics::draw(ctx, &center_point, graphics::DrawParam::default())?;
    graphics::draw(ctx, &alignment_vector, graphics::DrawParam::default())?;
    graphics::draw(ctx, &separation_vector, graphics::DrawParam::default())?;
    graphics::draw(ctx, &cohesion_vector, graphics::DrawParam::default())?;
    Ok(())
}