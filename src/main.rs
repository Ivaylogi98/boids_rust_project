use ggez::audio::SoundSource;
use ggez::conf::{Conf, WindowMode};
use ggez::event;
use ggez::filesystem;
use ggez::graphics;
use ggez::input;
use ggez::input::mouse;
use ggez::mint::Point2;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
use rand::rngs::ThreadRng;

use boids::entities::Bird;
use boids::entities::Obstacle;
use boids::assets::Assets;

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

    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;
        let assets =  Assets::new(ctx)?;
        let birds = vec![Bird::new(Point2{x: screen_width / 2.0, y: screen_height / 2.0}, 0 as f32)];
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
            bird_spawn_cooldown: 1.0 as f32
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
            if mouse::button_pressed(ctx, mouse::MouseButton::Left) && self.bird_spawn_cooldown <= 0.0{
                let mouse_position = mouse::position(ctx);

                let x = mouse_position.x;
                let y = mouse_position.y;

                let new_bird = Bird::new(Point2{x: x, y: y}, 0 as f32);
                self.birds.push(new_bird);
                self.bird_spawn_cooldown = 1.0;
            }    
            for bird in self.birds.iter_mut() {
                bird.update(seconds, &mut self.rng);
            }

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