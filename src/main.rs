use event::KeyCode;
use ggez::{audio::SoundSource, conf::FullscreenType, nalgebra::distance};
use ggez::conf::{Conf, WindowMode};
use ggez::event;
use ggez::filesystem;
use ggez::graphics;
use ggez::input;
use ggez::input::mouse;
use ggez::nalgebra::{ Point2, Vector2 };
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use graphics::MeshBuilder;
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
    random_movement_rule: bool,
    screen_width: f32,
    screen_height: f32,
    game_paused: bool,
    time_until_orient_update: f32,
    bird_spawn_cooldown: f32,
    pause: Pause,
    debug: bool
}

impl MainState {
    pub const ALIGNMENT_VIEW_DISTANCE: f32 = 80_f32;
    pub const SEPARATION_VIEW_DISTANCE: f32 = 20_f32;
    pub const COHESION_VIEW_DISTANCE: f32 = 80_f32;
    pub const MAX_SPEED: f32 = 3_f32;
    pub const MAX_STEERING_VELOCITY: f32 = 0.03_f32;
    pub const RANDOM_MOVEMENT: f32 = 0.05_f32;

    pub const ALIGNMENT_MODIFIER: f32 = 1.5;
    pub const SEPARATION_MODIFIER: f32 = 1.0;
    pub const COHESION_MODIFIER: f32 = 1.0;

    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;
        let assets =  Assets::new(ctx)?;
        let birds = vec![Bird::new(Point2::new(screen_width / 2.0, screen_height / 2.0), Vector2::new(0.1, 0.1))];
        let s = MainState {
            rng: rand::thread_rng(),
            assets: assets,
            birds: birds,
            obstacles: Vec::new(),
            input: InputState::default(),
            separation_rule: true,
            alignment_rule: true,
            cohesion_rule: true,
            random_movement_rule: true,
            screen_width: conf.window_mode.width,
            screen_height: conf.window_mode.height,
            game_paused: false,
            time_until_orient_update: 0.1 as f32,
            bird_spawn_cooldown: 0.1 as f32,
            pause: Pause::Running,
            debug: false
        };

        Ok(s)
    }

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
            "random" => {
                self.random_movement_rule = !self.random_movement_rule;
                println!("Random movement rule is {}", self.random_movement_rule);
            },
            "debug" => {
                self.debug = !self.debug;
                println!("Debug is {}", self.debug);
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

                    let new_bird = Bird::new(Point2::new(x*0.99, y*0.96), Vector2::new(self.rng.gen_range(-0.1 .. 0.1), self.rng.gen_range(-0.1 .. 0.1)) );
                    self.birds.push(new_bird);
                    self.bird_spawn_cooldown = 0.1;
                }
                for i in 0..self.birds.len() {

                    let mut acceleration: Vector2<f32> = Vector2::new(0.0, 0.0);

                    // ------------------------------------------ALIGNMENT RULE:--------------------------------------------
                    let mut velocity_sum_of_neigbours: Vector2<f32> = Vector2::new(0.0, 0.0);
                    let mut number_of_neighbours = 0;

                    for j in 0..self.birds.len() {
                        let distance: f32 = distance(&self.birds[i].pos, &self.birds[j].pos);
                        if distance > 0.0 && distance <= MainState::ALIGNMENT_VIEW_DISTANCE {
                            velocity_sum_of_neigbours += self.birds[j].vel;
                            number_of_neighbours += 1;
                        }
                    }

                    if number_of_neighbours > 0 {
                        velocity_sum_of_neigbours /= number_of_neighbours as f32;
                        // Tools::normalize_vector(&mut velocity_sum_of_neigbours);
                        // velocity_sum_of_neigbours *= MainState::MAX_SPEED;
                        // velocity_sum_of_neigbours -= self.birds[i].vel;
                        // Tools::limit_vector(&mut velocity_sum_of_neigbours, MainState::MAX_STEERING_VELOCITY);
                    }
                    else {
                        velocity_sum_of_neigbours = Vector2::new(0.0, 0.0);

                    }
                    if self.alignment_rule {
                        acceleration += (velocity_sum_of_neigbours / 8.0) * MainState::ALIGNMENT_MODIFIER;
                    }

                    // ----------------------------------------SEPARATION RULE:-----------------------------------------------
                    let mut steer_away_velocity: Vector2<f32> = Vector2::new(0.0, 0.0);

                    for j in 0..self.birds.len() {
                        let distance: f32 = distance(&self.birds[i].pos, &self.birds[j].pos);

                        if distance > 0.0 && distance <= MainState::SEPARATION_VIEW_DISTANCE {
                            let mut vector_away_from_neightbour: Vector2<f32> = self.birds[i].pos - self.birds[j].pos;
                            Tools::normalize_vector(&mut vector_away_from_neightbour);
                            vector_away_from_neightbour /= distance;
                            steer_away_velocity += vector_away_from_neightbour;
                            number_of_neighbours += 1;
                        }
                    }

                    if number_of_neighbours > 0 {
                        steer_away_velocity /= number_of_neighbours as f32;
                    }
                    if Tools::vector_length(&steer_away_velocity) > 0.0 {
                        Tools::normalize_vector(&mut steer_away_velocity);
                        // steer_away_velocity *= MainState::MAX_SPEED;
                        // steer_away_velocity -= self.birds[i].vel;
                        // Tools::limit_vector(&mut steer_away_velocity, MainState::MAX_STEERING_VELOCITY);
                    }

                    if self.separation_rule {
                        acceleration += steer_away_velocity * MainState::SEPARATION_MODIFIER;
                    }

                    // ------------------------------------------COHESION RULE:----------------------------------------------
                    let mut average_position: Point2<f32> = Point2::new(0.0, 0.0);
                    let mut number_of_neighbours = 0;

                    for j in 0..self.birds.len() {
                        let distance: f32 = distance(&self.birds[i].pos, &self.birds[j].pos);
                        if distance > 0.0 && distance <= MainState::COHESION_VIEW_DISTANCE {
                            average_position.x += self.birds[j].pos.x;
                            average_position.y += self.birds[j].pos.y;
                            number_of_neighbours += 1;
                        }
                    }

                    let mut steer_towards_velocity: Vector2<f32> = Vector2::new(0.0, 0.0);

                    if number_of_neighbours > 0 {
                        average_position /= number_of_neighbours as f32;
                        let mut vector_towards_average: Vector2<f32> = Tools::get_vec_from_to(self.birds[i].pos, average_position);
                        Tools::normalize_vector(&mut vector_towards_average);
                        vector_towards_average *= MainState::MAX_SPEED;

                        steer_towards_velocity = vector_towards_average - self.birds[i].vel;
                        Tools::limit_vector(&mut steer_towards_velocity, MainState::MAX_STEERING_VELOCITY);
                    }

                    if self.cohesion_rule {
                        acceleration += (steer_towards_velocity) * MainState::COHESION_MODIFIER;
                    }

                    // println!("alignment: {:?}", velocity_sum_of_neigbours);
                    // println!("separation: {:?}", steer_away_velocity);
                    // set birds that are out of the screen as not alive
                    // if self.birds[i].pos.x < 0.0 || self.birds[i].pos.x >= self.screen_width || 
                    //    self.birds[i].pos.y < 0.0 || self.birds[i].pos.y >= self.screen_height{
                    //        self.birds[i].is_alive = false;
                    //    }
                    // println!("{}:\nAlignment: {:?}\nSeparation: {:?}\nCohesion: {:?}", i, velocity_sum_of_neigbours, steer_away_velocity, steer_towards_velocity);

                    let random_movement: Vector2<f32> = Vector2::new(
                        self.rng.gen_range(-MainState::RANDOM_MOVEMENT .. MainState::RANDOM_MOVEMENT), 
                        self.rng.gen_range(-MainState::RANDOM_MOVEMENT .. MainState::RANDOM_MOVEMENT)
                    );
                    if self.random_movement_rule {
                        acceleration += random_movement;
                    }

                    self.birds[i].update(acceleration , MainState::MAX_SPEED, self.screen_width, self.screen_height);
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
            event::KeyCode::R => self.toggle_rule("random"),
            event::KeyCode::D => self.toggle_rule("debug"),
            event::KeyCode::K => {
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
            if self.debug == true || debug::is_active() {
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
            width: 1920.0,
            height: 1080.0,
            maximized: true,
            fullscreen_type: FullscreenType::Desktop,
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

    match event::run(&mut ctx, &mut event_loop, &mut state) {
        Err(e) => println!("ERROR in event::run -> {:?}", e),
        _ => ()
    }
}