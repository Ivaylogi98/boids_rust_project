use event::KeyCode;
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
use graphics::{Rect, MeshBuilder};
use nalgebra::allocator::Reallocator;
use rand::Rng;
use rand::rngs::ThreadRng;

use boids::entities::Bird;
use boids::entities::Obstacle;
use boids::assets::Assets;
use boids::debug;
use boids::tools::Tools;

use std::env;
use std::path;

#[derive(Debug, Default)]
struct InputState {
    movement: f32,
    fire: bool,
}
#[derive(Eq, PartialEq)]
enum Pause{
    Running,
    ToPause,
    Paused
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
    bird_spawn_cooldown: f32,
    pause: Pause
}

impl MainState {
    pub const ALIGNMENT_VIEW_DISTANCE: f32 = 80_f32;
    pub const SEPARATION_VIEW_DISTANCE: f32 = 25_f32;
    pub const COHESION_DISTANCE: f32 = 80_f32;
    pub const MAX_SPEED: f32 = 2_f32;
    pub const MAX_STEERING_VELOCITY: f32 = 0.5_f32;

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
            bird_spawn_cooldown: 0.1 as f32,
            pause: Pause::Running
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
            "separation" =>{
                self.separation_rule = !self.separation_rule;
                println!("Separation rule is {}", self.separation_rule);
            },
            "alignment" => {
                self.alignment_rule = !self.alignment_rule;
                println!("Alignment rule is {}", self.alignment_rule);
            },
            "cohesion" => {
                self.cohesion_rule = !self.cohesion_rule;
                println!("Cohesion rule is {}", self.cohesion_rule);
            },
            _ => ()
        }
    }
    fn toggle_pause(&mut self) {
        match self.pause {
            Pause::Running => self.pause = Pause::ToPause,
            Pause::ToPause => self.pause = Pause::Running,
            Pause::Paused => self.pause = Pause::Running
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
            if self.pause == Pause::Running {
                self.bird_spawn_cooldown -= seconds;

                if mouse::button_pressed(ctx, mouse::MouseButton::Left) && self.bird_spawn_cooldown <= 0.0 {
                    let mouse_position = mouse::position(ctx);

                    let x = mouse_position.x;
                    let y = mouse_position.y;

                    let new_bird = Bird::new(Point2{ x: x, y: y}, Vector2{ x: self.rng.gen_range(-0.1 .. 0.1), y: self.rng.gen_range(-0.1 .. 0.1) });
                    self.birds.push(new_bird);
                    self.bird_spawn_cooldown = 0.1;
                }
                // println!("{} - {:?}", self.birds.len(), self.birds);
                for i in 0..self.birds.len() {

                    let mut acceleration = Vector2{ x: 0.0, y: 0.0 };

                    // --------------------------------ALIGNMENT RULE:----------------------------------
                    let mut velocity_sum_of_neigbours = Vector2{ x: 0.0, y: 0.0 };
                    let mut number_of_neighbours = 0;

                    for j in 0..self.birds.len() {
                        let distance = Tools::distance(&self.birds[i], &self.birds[j]);
                        if distance > 0.0 && distance <= MainState::ALIGNMENT_VIEW_DISTANCE {
                            Tools::vec_op(&mut velocity_sum_of_neigbours, &self.birds[j].vel, |a,b| a + b);
                            number_of_neighbours += 1;
                        }
                    }

                    if number_of_neighbours > 0 {
                        Tools::vec_scalar_op(&mut velocity_sum_of_neigbours, number_of_neighbours as f32, |a, b| a / b);
                    }
                    if self.alignment_rule {
                        Tools::vec_op(&mut acceleration, &velocity_sum_of_neigbours, |a, b| a + b);
                    }

                    // --------------------------------SEPARATION RULE:-------------------------------
                    let mut steer_away_velocity = Vector2{x: 0.0, y: 0.0};
                    for j in 0..self.birds.len() {
                        let distance = Tools::distance(&self.birds[i], &self.birds[j]);
                        if distance > 0.0 && distance <= MainState::SEPARATION_VIEW_DISTANCE {
                            let mut vector_away_from_neightbour = Tools::get_vec_from_to(self.birds[i].pos, self.birds[j].pos);
                            Tools::normalize_vector(&mut vector_away_from_neightbour);
                            Tools::vec_scalar_op(&mut vector_away_from_neightbour, distance, |a,b| a / b);
                            Tools::vec_op(&mut steer_away_velocity, &vector_away_from_neightbour, |a,b| a + b);
                            number_of_neighbours += 1;
                        }
                    }
                    if number_of_neighbours > 0 {
                        Tools::vec_scalar_op(&mut steer_away_velocity, number_of_neighbours as f32, |a,b| a / b);
                    }
                    if Tools::vector_length(&steer_away_velocity) > 0.0 {
                        Tools::normalize_vector(&mut steer_away_velocity);
                        Tools::vec_scalar_op(&mut steer_away_velocity, MainState::MAX_SPEED, |a,b| a * b);
                        Tools::vec_op(&mut steer_away_velocity, &self.birds[i].vel, |a, b| a - b);
                        Tools::limit_velocity(&mut steer_away_velocity, MainState::MAX_STEERING_VELOCITY);
                    }
                    if self.separation_rule {
                        Tools::vec_op(&mut acceleration, &steer_away_velocity, |a, b| a + b);
                    }

                    // --------------------------------COHESION RULE:------------------------------------
                    let mut average_position = Point2{ x: 0.0, y: 0.0 };
                    let mut steer_towards_velocity = Vector2{ x: 0.0, y: 0.0 };
                    let mut number_of_neighbours = 0;
                    for j in 0..self.birds.len() {
                        let distance = Tools::distance(&self.birds[i], &self.birds[j]);
                        if distance > 0.0 && distance <= MainState::COHESION_DISTANCE {
                            Tools::point_op(&mut average_position, &self.birds[j].pos, |a, b| a + b);
                            number_of_neighbours += 1;
                        }
                    }
                    if number_of_neighbours > 0 {
                        Tools::point_scalar_op(&mut average_position, number_of_neighbours as f32, |a,b| a / b);
                        steer_towards_velocity = Tools::get_vec_from_to(self.birds[i].pos, average_position);
                        Tools::normalize_vector(&mut steer_towards_velocity);
                        Tools::vec_scalar_op(&mut steer_towards_velocity, MainState::MAX_SPEED, |a, b| a * b);
                        Tools::vec_op(&mut steer_towards_velocity, &self.birds[i].vel, |a, b| a - b);
                        Tools::limit_velocity(&mut steer_towards_velocity, MainState::MAX_STEERING_VELOCITY);
                    }
                    if self.cohesion_rule {
                        Tools::vec_op(&mut acceleration, &steer_towards_velocity, |a, b| a + b);
                    }

                    // println!("alignment: {:?}", velocity_sum_of_neigbours);
                    // println!("separation: {:?}", steer_away_velocity);
                    // set birds that are out of the screen as not alive
                    // if self.birds[i].pos.x < 0.0 || self.birds[i].pos.x >= self.screen_width || 
                    //    self.birds[i].pos.y < 0.0 || self.birds[i].pos.y >= self.screen_height{
                    //        self.birds[i].is_alive = false;
                    //    }
                    // println!("{}:\nAlignment: {:?}\nSeparation: {:?}\nCohesion: {:?}", i, velocity_sum_of_neigbours, steer_away_velocity, steer_towards_velocity);
                    self.birds[i].update(acceleration, MainState::MAX_SPEED, self.screen_width, self.screen_height);
                }
                // remove birds that are not alive
                self.birds.retain(|bird| bird.is_alive);
                    
            }   

        }

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: event::KeyCode, _keymod: input::keyboard::KeyMods, _repeat: bool) {
        match keycode {
            event::KeyCode::S => self.toggle_rule("separation"),
            event::KeyCode::A => self.toggle_rule("alignment"),
            event::KeyCode::C => self.toggle_rule("cohesion"),
            event::KeyCode::R => {
                for bird in self.birds.iter_mut() {
                    bird.is_alive = false;
                }
            },
            event::KeyCode::P => self.toggle_pause(),
            event::KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dark_blue = graphics::Color::from_rgb(26, 51, 77);
        if self.pause == Pause::Running {
            graphics::clear(ctx, dark_blue);
            for bird in self.birds.iter_mut() {
                bird.draw(ctx, &self.assets)?;
            }
            if debug::is_active() {
                for bird in &mut self.birds {
                    debug::draw_debug_info(
                        bird.alignment_view_distance_circle(ctx, MainState::ALIGNMENT_VIEW_DISTANCE),
                        bird.separation_view_distance_circle(ctx, MainState::SEPARATION_VIEW_DISTANCE),
                        bird.center_point(ctx), ctx).
                    unwrap();
                }
            }
            graphics::present(ctx)?;
        }
        else if self.pause == Pause::ToPause{
            self.pause = Pause::Paused;
            let pause_screen = MeshBuilder::new().rectangle(graphics::DrawMode::fill(), graphics::Rect::new(0.0, 0.0, self.screen_width, self.screen_height), (0, 0, 0, 60).into()).build(ctx).unwrap();
            graphics::draw(ctx, &pause_screen, graphics::DrawParam::default())?;
            graphics::present(ctx)?;
        }
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


        // if debug::is_active() {
        //     for obstacles in &mut self.obstacles {
        //         debug::draw_outline(enemy.bounding_rect(ctx), ctx).unwrap();
        //     }
        // }

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