use ggez::audio::SoundSource;
use ggez::conf::{Conf, WindowMode};
use ggez::event;
use ggez::filesystem;
use ggez::graphics;
use ggez::input;
use ggez::input::mouse;
use ggez::mint::{ Point2, Vector2 };
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
use rand::rngs::ThreadRng;

use boids::entities::Bird;
use boids::entities::Obstacle;
use boids::assets::Assets;
use boids::debug;

use std::env;
use std::path;

#[derive(Debug, Default)]
struct InputState {
    movement: f32,
    fire: bool,
}

struct MainState {
    rng: ThreadRng,
    assets: Assets,
    birds: Vec<Bird>,
    obstacles: Vec<Obstacle>,
    input: InputState,
    separation_rule: bool,
    alignment_rule: bool,
    cohesion_rule: bool,
    screen_width: f32,
    screen_height: f32,
    game_paused: bool,
    time_until_orient_update: f32,
    bird_spawn_cooldown: f32
}

impl MainState {
    pub const VIEW_DISTANCE: f32 = 200 as f32;
    pub const ALIGNMENT_MODIFIER: Vector2<f32> = Vector2{ x: 1.0, y: 1.0 };
    pub const SEPARATION_MODIFIER: Vector2<f32> = Vector2{ x: 1.0, y: 1.0 };
    pub const COHESION_MODIFIER: Vector2<f32> = Vector2{ x: 1.0, y: 1.0 };

    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;
        let assets =  Assets::new(ctx)?;
        let birds = vec![Bird::new(Point2{ x: screen_width / 2.0, y: screen_height / 2.0 }, Vector2{ x: 0.1, y: 0.1 })];
        let s = MainState {
            rng: rand::thread_rng(),
            assets: assets,
            birds: birds,
            obstacles: Vec::new(),
            input: InputState::default(),
            separation_rule: true,
            alignment_rule: true,
            cohesion_rule: true,
            screen_width: conf.window_mode.width,
            screen_height: conf.window_mode.height,
            game_paused: false,
            time_until_orient_update: 0.1 as f32,
            bird_spawn_cooldown: 0.5 as f32
        };

        Ok(s)
    }

    // fn handle_collisions(&mut self, ctx: &mut Context) {
    //     for obstacle in &mut self.obstacles {
    //         for bird in &mut self.birds {
    //             if obstacle.bounding_rect(ctx).contains(bird.pos) {
    //                 todo!();
    //             }
    //         }
    //     }
    // }

    fn toggle_rule(&mut self, rule: &str) {
        match rule {
            "separation" => self.separation_rule = !self.separation_rule,
            "alignment" => self.separation_rule = !self.alignment_rule,
            "cohesion" => self.separation_rule = !self.cohesion_rule,
            _ => ()
        }
    }



}
fn normalize_vector( vec: &mut Vector2<f32> ) {
    let length = (vec.x.powf(2.0) + vec.y.powf(2.0)).sqrt();
    vec.x /= length;
    vec.y /= length;
}
fn distance(b1: &Bird, b2: &Bird) -> f32 {
    ((b1.pos.x - b2.pos.x).powf(2.0) + (b1.pos.y - b2.pos.y).powf(2.0)).sqrt()
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // if self.game_over {
        //     return Ok(());
        // }

        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);


            // for obstacle in self.obstacles.iter_mut() {
            //     obstacle.update(seconds, self.rng);
            // }

            self.bird_spawn_cooldown -= seconds;

            if mouse::button_pressed(ctx, mouse::MouseButton::Left) && self.bird_spawn_cooldown <= 0.0 {
                let mouse_position = mouse::position(ctx);

                let x = mouse_position.x;
                let y = mouse_position.y;

                let new_bird = Bird::new(Point2{ x: x, y: y}, Vector2{ x: -0.1, y: -0.1 });
                self.birds.push(new_bird);
                self.bird_spawn_cooldown = 0.5;
            }

            for i in 0..self.birds.len() {

                // ALIGNMENT RULE:
                let mut velocity_sum_of_neigbours = Vector2{ x: 0.0, y: 0.0 };
                let mut number_of_neighbours = 0;

                for j in 0..self.birds.len() {
                    if distance(&self.birds[i], &self.birds[j]) <= MainState::VIEW_DISTANCE {
                        velocity_sum_of_neigbours.x += self.birds[j].vel.x;
                        velocity_sum_of_neigbours.y += self.birds[j].vel.y;
                        number_of_neighbours += 1;
                    }
                }

                if number_of_neighbours > 0 {
                    velocity_sum_of_neigbours.x /= number_of_neighbours as f32;
                    velocity_sum_of_neigbours.y /= number_of_neighbours as f32;
                }
                // SEPARATION RULE:

                let acceleration = velocity_sum_of_neigbours;
                // set birds that are out of the screen as not alive
                // if self.birds[i].pos.x < 0.0 || self.birds[i].pos.x >= self.screen_width || 
                //    self.birds[i].pos.y < 0.0 || self.birds[i].pos.y >= self.screen_height{
                //        self.birds[i].is_alive = false;
                //    }
                self.birds[i].update(acceleration, self.screen_width, self.screen_height);
            }
            // remove birds that are not alive
            self.birds.retain(|bird| bird.is_alive);

        }

        Ok(())
    }

    fn key_down_event(&mut self,
                      ctx: &mut Context,
                      keycode: event::KeyCode,
                      _keymod: input::keyboard::KeyMods,
                      _repeat: bool) {
        match keycode {
            event::KeyCode::Q => self.toggle_rule("separation"),
            event::KeyCode::W => self.toggle_rule("alignment"),
            event::KeyCode::E => self.toggle_rule("cohesion"),
            event::KeyCode::R => {
                for bird in self.birds.iter_mut() {
                    bird.is_alive = false;
                }
            },
            event::KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self,
                    _ctx: &mut Context,
                    keycode: event::KeyCode,
                    _keymod: input::keyboard::KeyMods) {
        match keycode {
            event::KeyCode::Space => self.input.fire = false,
            event::KeyCode::Left | event::KeyCode::Right => self.input.movement = 0.0,
            _ => (), // Do nothing
        }
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dark_blue = graphics::Color::from_rgb(26, 51, 77);
        graphics::clear(ctx, dark_blue);

        // if self.game_over {
        //     let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        //     let mut text = graphics::Text::new(format!("Killed by {}.\nScore: {}", self.killed_by, self.score));
        //     text.set_font(font, graphics::PxScale::from(40.0));

        //     let top_left = Point2 {
        //         x: (self.screen_width - text.width(ctx)) / 2.0,
        //         y: (self.screen_height - text.height(ctx)) / 2.0,
        //     };
        //     graphics::draw(ctx, &text, graphics::DrawParam {
        //         dest: top_left,
        //         .. Default::default()
        //     })?;
        //     graphics::present(ctx)?;
        //     return Ok(())
        // }

        // for obstacle in self.obstacles.iter_mut() {
        //     obstacle.draw(ctx)?;
        // }

        for bird in self.birds.iter_mut() {
            bird.draw(ctx, &self.assets)?;
        }
        if debug::is_active() {
            for bird in &mut self.birds {
                debug::draw_debug_info(bird.view_distance_circle(ctx, MainState::VIEW_DISTANCE), bird.center_point(ctx), ctx).unwrap();
            }
        }
        // if debug::is_active() {
        //     for obstacles in &mut self.obstacles {
        //         debug::draw_outline(enemy.bounding_rect(ctx), ctx).unwrap();
        //     }
        // }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() {
    let conf = Conf::new().
        window_mode(WindowMode {
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        });
    let (mut ctx, mut event_loop) = ContextBuilder::new("boids", "Ivaylogi").conf(conf.clone()).build().unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(&mut ctx, &path, true);
    }

    let mut state = MainState::new(&mut ctx, &conf).unwrap();

    event::run(&mut ctx, &mut event_loop, &mut state);
}