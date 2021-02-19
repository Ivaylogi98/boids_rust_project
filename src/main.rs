use ggez::{conf::FullscreenType, nalgebra::distance};
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
enum Entity{
    Bird,
    Obstacle
}
struct MainState {
    rng: ThreadRng,
    assets: Assets,
    birds: Vec<Bird>,
    obstacles: Vec<Obstacle>,
    separation_rule: bool,
    alignment_rule: bool,
    cohesion_rule: bool,
    random_movement_rule: bool,
    screen_width: f32,
    screen_height: f32,
    spawn_cooldown: f32,
    pause: Pause,
    debug_circles: bool,
    debug_vectors: bool,
    spawn_entity: Entity
}

impl MainState {
    pub const ALIGNMENT_VIEW_DISTANCE: f32 = 100_f32;
    pub const SEPARATION_VIEW_DISTANCE: f32 = 30_f32;
    pub const COHESION_VIEW_DISTANCE: f32 = 100_f32;
    pub const OBSTACLE_RADIUS: f32 = 50.0;

    pub const MAX_SPEED: f32 = 3.5_f32;
    pub const MAX_STEERING_VELOCITY: f32 = 0.16_f32;
    pub const RANDOM_MOVEMENT: f32 = 0.1_f32;

    pub const ALIGNMENT_MODIFIER: f32 = 1.6;
    pub const SEPARATION_MODIFIER: f32 = 2.0;
    pub const COHESION_MODIFIER: f32 = 1.0;
    pub const OBSTACLE_MODIFIER: f32 = 2.5;

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
            separation_rule: true,
            alignment_rule: true,
            cohesion_rule: true,
            random_movement_rule: true,
            screen_width: conf.window_mode.width,
            screen_height: conf.window_mode.height,
            spawn_cooldown: 0.05 as f32,
            pause: Pause::Running,
            debug_circles: false,
            debug_vectors: false,
            spawn_entity: Entity::Bird
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
            "debug_circles" => {
                self.debug_circles = !self.debug_circles;
                println!("Debug circles is {}", self.debug_circles);
            },
            "debug_vectors" => {
                self.debug_vectors = !self.debug_vectors;
                println!("Debug vectors is {}", self.debug_vectors);
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
    fn toggle_spawn(&mut self) {
        match self.spawn_entity {
            Entity::Bird => self.spawn_entity = Entity::Obstacle,
            Entity::Obstacle => self.spawn_entity = Entity::Bird
        }
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            if self.pause == Pause::Running {
                self.spawn_cooldown -= seconds;

                if mouse::button_pressed(ctx, mouse::MouseButton::Left) && self.spawn_cooldown <= 0.0 {
                    let mouse_position = mouse::position(ctx);

                    let x = mouse_position.x;
                    let y = mouse_position.y;

                    match self.spawn_entity {
                        Entity::Bird => {
                            let new_bird = Bird::new(Point2::new(x*0.99, y*0.96), Vector2::new(self.rng.gen_range(-0.1 .. 0.1), self.rng.gen_range(-0.1 .. 0.1)) );
                            self.birds.push(new_bird);
                            self.spawn_cooldown = 0.05;
                        },
                        Entity::Obstacle => {
                            let new_obstacle = Obstacle::new(Point2::new(x*0.99, y*0.96), MainState::OBSTACLE_RADIUS);
                            self.obstacles.push(new_obstacle);
                            self.spawn_cooldown = 0.05;
                        }
                    }
                }
                for i in 0..self.birds.len() {

                    // ------------------------------------------ALIGNMENT RULE:--------------------------------------------
                    let mut velocity_sum_of_neigbours: Vector2<f32> = Vector2::new(0.0, 0.0);
                    let mut number_of_neighbours = 0;

                    if self.alignment_rule {
                        for j in 0..self.birds.len() {
                            let distance: f32 = distance(&self.birds[i].pos, &self.birds[j].pos);
                            if distance > 0.0 && distance <= MainState::ALIGNMENT_VIEW_DISTANCE {
                                velocity_sum_of_neigbours += self.birds[j].vel;
                                number_of_neighbours += 1;
                            }
                        }

                        if number_of_neighbours > 0 {
                            velocity_sum_of_neigbours /= number_of_neighbours as f32;
                            Tools::normalize_vector(&mut velocity_sum_of_neigbours);
                            velocity_sum_of_neigbours *= MainState::MAX_SPEED;
                            velocity_sum_of_neigbours -= self.birds[i].vel;
                            Tools::limit_vector(&mut velocity_sum_of_neigbours, MainState::MAX_STEERING_VELOCITY);
                        }
                        else {
                            velocity_sum_of_neigbours = Vector2::new(0.0, 0.0);

                        }
                        velocity_sum_of_neigbours *= MainState::ALIGNMENT_MODIFIER;
                    }

                    // ----------------------------------------SEPARATION RULE:-----------------------------------------------
                    let mut steer_away_velocity: Vector2<f32> = Vector2::new(0.0, 0.0);
                    let mut number_of_neighbours = 0;

                    if self.separation_rule {
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
                            steer_away_velocity *= MainState::MAX_SPEED;
                            steer_away_velocity -= self.birds[i].vel;
                            Tools::limit_vector(&mut steer_away_velocity, MainState::MAX_STEERING_VELOCITY);
                        }
                        steer_away_velocity *= MainState::SEPARATION_MODIFIER;
                    }

                    // ------------------------------------------COHESION RULE:----------------------------------------------
                    let mut average_position: Point2<f32> = Point2::new(0.0, 0.0);
                    let mut number_of_neighbours = 0;
                    let mut steer_towards_velocity: Vector2<f32> = Vector2::new(0.0, 0.0);

                    if self.cohesion_rule {
                        for j in 0..self.birds.len() {
                            let distance: f32 = distance(&self.birds[i].pos, &self.birds[j].pos);
                            if distance > 0.0 && distance <= MainState::COHESION_VIEW_DISTANCE {
                                average_position.x += self.birds[j].pos.x;
                                average_position.y += self.birds[j].pos.y;
                                number_of_neighbours += 1;
                            }
                        }

                        if number_of_neighbours > 0 {
                            average_position /= number_of_neighbours as f32;
                            let mut vector_towards_average: Vector2<f32> = Tools::get_vec_from_to(average_position, self.birds[i].pos);
                            Tools::normalize_vector(&mut vector_towards_average);
                            vector_towards_average *= MainState::MAX_SPEED;

                            steer_towards_velocity = vector_towards_average - self.birds[i].vel;
                            Tools::limit_vector(&mut steer_towards_velocity, MainState::MAX_STEERING_VELOCITY);
                        }
                        steer_towards_velocity *= MainState::COHESION_MODIFIER;
                    }

                    // ------------------------------------------RANDOM MOVEMENT:----------------------------------------------
                    let mut random_movement: Vector2<f32> = Vector2::new(0.0, 0.0);
                    if self.random_movement_rule {
                        random_movement = Vector2::new(
                            self.rng.gen_range(-MainState::RANDOM_MOVEMENT .. MainState::RANDOM_MOVEMENT), 
                            self.rng.gen_range(-MainState::RANDOM_MOVEMENT .. MainState::RANDOM_MOVEMENT)
                        );
                    }

                    // ------------------------------------------OBSTACLE EVASION:----------------------------------------------
                    let mut obstacle_evasion: Vector2<f32> = Vector2::new(0.0, 0.0);
                    for obstacle in self.obstacles.iter() {
                        let distance: f32 = distance(&self.birds[i].pos, &obstacle.pos);
                        if distance <= MainState::OBSTACLE_RADIUS {
                            let mut vector_away_from_obstacle: Vector2<f32> = self.birds[i].pos - obstacle.pos;
                            Tools::normalize_vector(&mut vector_away_from_obstacle);
                            vector_away_from_obstacle /= distance;
                            obstacle_evasion += vector_away_from_obstacle;
                            number_of_neighbours += 1;
                        }
                    }
                    if number_of_neighbours > 0 {
                        obstacle_evasion /= number_of_neighbours as f32;
                    }
                    if Tools::vector_length(&obstacle_evasion) > 0.0 {
                        Tools::normalize_vector(&mut obstacle_evasion);
                        obstacle_evasion *= MainState::MAX_SPEED;
                        obstacle_evasion -= self.birds[i].vel;
                        Tools::limit_vector(&mut obstacle_evasion, MainState::MAX_STEERING_VELOCITY);
                    }
                    obstacle_evasion *= MainState::OBSTACLE_MODIFIER;
                    // ---------------------------------------------------------------------------------------------------------
                    
                    self.birds[i].update(
                        velocity_sum_of_neigbours,
                        steer_away_velocity,  
                        steer_towards_velocity,
                        random_movement,
                        obstacle_evasion,
                        MainState::MAX_SPEED,
                        self.screen_width, self.screen_height);
                }
                // remove entities that are not alive
                self.birds.retain(|bird| bird.is_alive);
                self.obstacles.retain(|obstacle| obstacle.is_alive);
                    
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
            event::KeyCode::D => self.toggle_rule("debug_circles"),
            event::KeyCode::V => self.toggle_rule("debug_vectors"),
            event::KeyCode::P => self.toggle_pause(),
            event::KeyCode::Space => self.toggle_spawn(),
            event::KeyCode::Escape => event::quit(ctx),
            event::KeyCode::B => {
                for bird in self.birds.iter_mut() {
                    bird.is_alive = false;
                }
            },
            event::KeyCode::O => {
                for obstacle in self.obstacles.iter_mut() {
                    obstacle.is_alive = false;
                }
            },
            _ => (), // Do nothing
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        
        let background = graphics::Color::from_rgb(30, 35, 56);

        if self.pause == Pause::Running {
            graphics::clear(ctx, background);
            
            // draw entities
            for bird in self.birds.iter_mut() {
                bird.draw(ctx, &self.assets)?;
            }

            for obstacle in self.obstacles.iter_mut() {
                obstacle.draw(ctx, &self.assets)?;
            }

            if self.debug_circles || self.debug_vectors || debug::is_active() {
                for bird in &mut self.birds {
                    if self.debug_circles{
                        debug::draw_debug_circles(
                            bird.alignment_view_distance_circle(ctx, MainState::ALIGNMENT_VIEW_DISTANCE),
                            bird.separation_view_distance_circle(ctx, MainState::SEPARATION_VIEW_DISTANCE),
                            bird.center_point(ctx),
                            ctx).
                        unwrap();
                    }
                    if self.debug_vectors {
                        debug::draw_debug_vectors(
                            bird.alignment_vector(ctx),
                            bird.separation_vector(ctx),
                            bird.cohesion_vector(ctx),
                            bird.obstacle_vector(ctx),
                            ctx).
                        unwrap();
                    }
                    // println!("{:?}", bird);
                }
            }

            // draw UI
            // draw alignment rule text
            let drawparams = graphics::DrawParam::new().scale(Vector2::new(1.0, 1.0)).offset(Point2::new(0.0, 0.0));

            let new_drawarams = if self.alignment_rule {
                drawparams.color((0, 255, 0).into())
            }
            else {
                drawparams.color((255, 0, 0).into())
            }.dest(Point2::new(0.0, self.screen_height / 2.0));

            graphics::draw(ctx, &graphics::Text::new("alignment"), new_drawarams)?;

            // draw separation rule text
            let new_drawarams = if self.separation_rule {
                drawparams.color((0, 255, 0).into())
            }
            else {
                drawparams.color((255, 0, 0).into())
            }.dest(Point2::new(0.0, self.screen_height / 2.0 + 20.0));

            graphics::draw(ctx, &graphics::Text::new("separation"), new_drawarams)?;

            // draw cohesion rule text
            let new_drawarams = if self.cohesion_rule {
                drawparams.color((0, 255, 0).into())
            }
            else {
                drawparams.color((255, 0, 0).into())
            }.dest(Point2::new(0.0, self.screen_height / 2.0 + 40.0));

            graphics::draw(ctx, &graphics::Text::new("cohesion"), new_drawarams)?;

            // draw random movement text
            let new_drawarams = if self.random_movement_rule {
                drawparams.color((0, 255, 0).into())
            }
            else {
                drawparams.color((255, 0, 0).into())
            }.dest(Point2::new(0.0, self.screen_height / 2.0 + 60.0));

            graphics::draw(ctx, &graphics::Text::new("random movement"), new_drawarams)?;
            
            // draw birds count text
            let new_drawarams = if self.birds.len() > 0 {
                drawparams.color((0, 255, 0).into())
            }
            else {
                drawparams.color((255, 0, 0).into())
            }.dest(Point2::new(0.0, self.screen_height / 2.0 + 90.0));
            graphics::draw(ctx, &graphics::Text::new(format!("Birds:{}", self.birds.len())), new_drawarams)?;

            // draw obstacles count text
            let new_drawarams = if self.obstacles.len() > 0 {
                drawparams.color((0, 255, 0).into())
            }
            else {
                drawparams.color((255, 0, 0).into())
            }.dest(Point2::new(0.0, self.screen_height / 2.0 + 110.0));

            graphics::draw(ctx, &graphics::Text::new(format!("Obstacles:{}", self.obstacles.len())), new_drawarams)?;
            

            graphics::present(ctx)?;
        }
        else if self.pause == Pause::ToPause{
            self.pause = Pause::Paused;
            let pause_screen = MeshBuilder::new().rectangle(
                graphics::DrawMode::fill(), 
                graphics::Rect::new(0.0, 0.0, self.screen_width, self.screen_height), 
                (0, 0, 0, 60).into()).build(ctx).unwrap();

            graphics::draw(ctx, &pause_screen, graphics::DrawParam::default())?;
            
            // draw menu on pause screen
            let drawparams = graphics::DrawParam::new()
                                    .dest(Point2::new(self.screen_width / 2.0 - 200.0, self.screen_height / 2.0 - 60.0))
                                    .scale(Vector2::new(1.2, 1.2));
            let pause_menu_legend = r"Press:
    ESC to exit
    SPACE to toggle entity spawning (bird / obstacle)
    O to remove obstacles
    B to remove birds
    P to pause and unpause
    D to show view distances
    V to show vectors";
            graphics::draw(ctx, &graphics::Text::new(pause_menu_legend), drawparams)?;

            graphics::present(ctx)?;
        }

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