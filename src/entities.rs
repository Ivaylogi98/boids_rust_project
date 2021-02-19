use core::time;

use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::{Mesh, MeshBuilder, DrawMode};
use ggez::nalgebra::{Point2, Vector2};
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::assets::Assets;
use crate::tools::Tools;


#[derive(Debug, Copy, Clone)]
pub struct Bird {
    pub pos: Point2<f32>,
    pub vel: Vector2<f32>,
    pub align: Vector2<f32>,
    pub sep: Vector2<f32>,
    pub coh: Vector2<f32>,
    pub obst: Vector2<f32>,
    pub random: Vector2<f32>,
    pub is_alive: bool
}

impl Bird{
    pub const SELF_ACCELERATION: f32 = 1.0;

    pub fn new(pos: Point2<f32>, vel: Vector2<f32>) -> Self {
        Bird{
            pos: pos,
            vel: vel,
            align: Vector2::new(0.0, 0.0),
            sep: Vector2::new(0.0, 0.0),
            coh: Vector2::new(0.0, 0.0),
            obst: Vector2::new(0.0, 0.0),
            random: Vector2::new(0.0, 0.0),
            is_alive: true
        }
    }

    pub fn update(&mut self, align: Vector2<f32>, sep: Vector2<f32>, coh: Vector2<f32>, random: Vector2<f32>, obst: Vector2<f32>, max_velocity: f32, screen_width: f32, screen_height: f32) {
        // update vectors in self
        self.align = align;
        self.sep = sep;
        self.coh = coh;
        self.random = random;
        self.obst = obst;

        // update velocity
        let acceleration: Vector2<f32> = align + sep + coh + random;
        if acceleration.x == 0.0 && acceleration.y == 0.0 {
            self.vel *= Bird::SELF_ACCELERATION;
        }
        else {
            self.vel += acceleration;
        }
        Tools::limit_vector(&mut self.vel, max_velocity);
        self.vel += obst;
        Tools::limit_vector(&mut self.vel, max_velocity);

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
                                .scale(Vector2::new(0.05, 0.05))
                                .offset(Point2::new(0.47, 0.7))
                                .rotation((self.vel.y).atan2(self.vel.x) + 3.1415/2.0);
        graphics::draw(ctx, &assets.bird, drawparams)
    }


    pub fn alignment_view_distance_circle(&self, ctx: &mut Context, alignment_view_distance: f32) -> graphics::Mesh {
        MeshBuilder::new().circle(
            graphics::DrawMode::stroke(1.0), 
            Point2::new(self.pos.x, self.pos.y), 
            alignment_view_distance, 
            1.0, 
            (255, 0, 0).into()).build(ctx).unwrap()
    }
    pub fn separation_view_distance_circle(&self, ctx: &mut Context, separation_view_distance: f32) -> graphics::Mesh {
        MeshBuilder::new().circle(
            graphics::DrawMode::stroke(1.0), 
            Point2::new(self.pos.x, self.pos.y), 
            separation_view_distance, 
            1.0, 
            (255, 0, 0).into()).build(ctx).unwrap()
    }
    pub fn center_point(&self, ctx: &mut Context) -> graphics::Mesh {
        MeshBuilder::new().circle(
            graphics::DrawMode::fill(), 
            Point2::new(self.pos.x, self.pos.y), 
            4.0, 
            1.0, 
            (255, 0, 0).into()).build(ctx).unwrap()
    }
    pub fn alignment_vector(&self, ctx: &mut Context) -> graphics::Mesh {
        if self.align.x == 0.0 && self.align.y == 0.0 {
            Mesh::new_circle(ctx, DrawMode::fill(), self.pos, 1.0, 1.0, (255, 0, 0).into()).unwrap()
        }
        else {
            Mesh::new_line(ctx, &[self.pos, Point2::new(self.pos.x + self.align.x * 1000.0, self.pos.y + self.align.y * 1000.0)], 1.0, (255, 0, 0).into()).unwrap()
        }
    }
    pub fn separation_vector(&self, ctx: &mut Context) -> graphics::Mesh {
        if self.sep.x == 0.0 && self.sep.y == 0.0 {
            Mesh::new_circle(ctx, DrawMode::fill(), self.pos, 1.0, 1.0, (255, 0, 0).into()).unwrap()
        }
        else {
            Mesh::new_line(ctx, &[self.pos, Point2::new(self.pos.x + self.sep.x * 1000.0, self.pos.y + self.sep.y * 1000.0)], 1.0, (0, 255, 0).into()).unwrap()
        }
    }
    pub fn cohesion_vector(&self, ctx: &mut Context) -> graphics::Mesh {
        if self.coh.x == 0.0 && self.coh.y == 0.0 {
            Mesh::new_circle(ctx, DrawMode::fill(), self.pos, 1.0, 1.0, (255, 0, 0).into()).unwrap()
        }
        else {
            Mesh::new_line(ctx, &[self.pos, Point2::new(self.pos.x + self.coh.x * 1000.0, self.pos.y + self.coh.y * 1000.0)], 1.0, (0, 0, 255).into()).unwrap()
        }
    }
}

#[derive(Debug)]
pub struct Obstacle {
    pub pos: Point2<f32>,
    pub radius: f32,
    pub is_alive: bool
}

impl Obstacle{

    pub fn new(pos: Point2<f32>, radius: f32) -> Self {
        Obstacle{
            pos: pos,
            radius: radius,
            is_alive: true
        }
    }
    pub fn update(&mut self) {
        todo!()        
    }
    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        let drawparams = graphics::DrawParam::new().
                                    scale(Vector2::new(0.1, 0.1)).
                                    offset(Point2::new(0.5, 0.5)).
                                    dest(self.pos);
        graphics::draw(ctx, &assets.obstacle, drawparams)
    }
}