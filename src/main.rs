use std::collections::HashMap;

use audio::{DEATH_SFX, JUMP_SFX};
use components::{CInput, CTransform};
use entities::{
    entity::Entity, entity_manager::EntityManager, entity_memory_pool::EntityMemoryPool, EntityTag,
};
use levels::{Level, PlatformRect};
use once_cell::sync::Lazy;
use sfml::{
    audio::Sound,
    graphics::{Color, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2,
    window::{mouse, Event, Key, Style, VideoMode},
};

use crate::audio::LEVEL_COMPLETE_SFX;

mod audio;
mod components;
mod entities;
mod levels;

const FRAMERATE: u32 = 60;

pub static SCREEN_WIDTH: Lazy<u32> = Lazy::new(|| VideoMode::desktop_mode().width);
pub static SCREEN_HEIGHT: Lazy<u32> = Lazy::new(|| VideoMode::desktop_mode().height);

struct World {
    entity_manager: EntityManager,
    entity_memory_pool: EntityMemoryPool,
    player: Entity,
    player_window: RenderWindow,
    player_spawn_position: Vector2<f32>,
    window_map: HashMap<usize, RenderWindow>,
    jump_input_timer: u32,
    jump_input_duration: u32,
    grounded_timer: u32,
    grounded_duration: u32,
    wall_contact_timer: i32,
    wall_contact_duration: u32,
    player_acceleration_scale: Vector2<f32>,
    goal: Entity,
    goal_window: RenderWindow,
    level_index: i32,
    death_sfx: Sound<'static>,
    level_complete_sfx: Sound<'static>,
    jump_sfx: Sound<'static>,
    wall_jump_physics: bool,
    levels: Vec<Level>,
}

impl World {
    pub fn new() -> Self {
        let mut entity_memory_pool = EntityMemoryPool::new(20000);
        let mut entity_manager = EntityManager::new();

        let mut window_map: HashMap<usize, RenderWindow> = HashMap::new();

        // Player
        let (player, player_window) = {
            let mut player = entity_manager.add_entity(EntityTag::Player, &mut entity_memory_pool);
            player.add_component::<CInput>(&mut entity_memory_pool);
            player.add_component::<CTransform>(&mut entity_memory_pool);
            let player_transform = player
                .get_component_mut::<CTransform>(&mut entity_memory_pool)
                .unwrap();

            player_transform.position =
                Vector2::new(*SCREEN_WIDTH as f32 / 2.0, *SCREEN_HEIGHT as f32 / 2.0);
            player_transform.size = Vector2::new(25.0, 50.0 as f32);
            player_transform.half_size = player_transform.size / 2.0;
            player_transform.max_speed = 15.0;

            let mut player_window = RenderWindow::new(
                VideoMode::new(
                    player_transform.size.x as u32,
                    player_transform.size.y as u32,
                    32,
                ),
                "Player",
                Style::NONE,
                &Default::default(),
            );
            player_window.set_framerate_limit(FRAMERATE);
            player_window.set_position(Vector2::new(
                (*SCREEN_WIDTH / 2 - player_window.size().x) as i32,
                (*SCREEN_HEIGHT / 2 - player_window.size().y) as i32,
            ));

            (player, player_window)
        };

        // Goal
        let (goal, goal_window) = {
            let mut goal = entity_manager.add_entity(EntityTag::Goal, &mut entity_memory_pool);
            goal.add_component::<CTransform>(&mut entity_memory_pool);
            let goal_transform = goal
                .get_component_mut::<CTransform>(&mut entity_memory_pool)
                .unwrap();

            goal_transform.size = Vector2::new(50.0, 50.0);
            goal_transform.half_size = goal_transform.size / 2.0;
            goal_transform.position =
                Vector2::new(*SCREEN_WIDTH as f32 / 2.0, goal_transform.size.y);

            let mut goal_window = RenderWindow::new(
                VideoMode::new(
                    goal_transform.size.x as u32,
                    goal_transform.size.y as u32,
                    32,
                ),
                "Goal",
                Style::NONE,
                &Default::default(),
            );
            goal_window.set_framerate_limit(FRAMERATE);
            goal_window.set_position(Vector2::new(
                goal_transform.position.x as i32 - goal_transform.half_size.x as i32,
                goal_transform.position.y as i32 - goal_transform.half_size.y as i32,
            ));

            (goal, goal_window)
        };

        Self {
            entity_manager,
            entity_memory_pool,
            player,
            player_window,
            player_spawn_position: Vector2::new(
                *SCREEN_WIDTH as f32 / 2.0,
                *SCREEN_HEIGHT as f32 / 2.0,
            ),
            window_map,
            jump_input_timer: 0,
            jump_input_duration: 6,
            grounded_timer: 0,
            grounded_duration: 6,
            wall_contact_timer: 0,
            wall_contact_duration: 10,
            player_acceleration_scale: Vector2::new(0.2, 0.5),
            goal,
            goal_window,
            level_index: -1,
            death_sfx: Sound::with_buffer(unsafe { &DEATH_SFX }),
            level_complete_sfx: Sound::with_buffer(unsafe { &LEVEL_COMPLETE_SFX }),
            jump_sfx: Sound::with_buffer(unsafe { &JUMP_SFX }),
            wall_jump_physics: false,
            levels: levels::get_levels(),
        }
    }

    pub fn s_update(&mut self) {
        if self.level_index == -1 {
            self.level_index = 0;
            self.load_level(self.level_index as u32);
        }

        // Update entities
        self.entity_manager.update(&mut self.entity_memory_pool);

        // Reset input component
        {
            let player_input = self
                .player
                .get_component_mut::<CInput>(&mut self.entity_memory_pool)
                .unwrap();

            player_input.up.pressed = false;
            player_input.up.released = false;
            player_input.down.pressed = false;
            player_input.down.released = false;
            player_input.left.pressed = false;
            player_input.left.released = false;
            player_input.right.pressed = false;
            player_input.right.released = false;
            player_input.space.pressed = false;
            player_input.space.released = false;
        }

        // Poll events
        while let Some(event) = self.player_window.poll_event() {
            match event {
                Event::Closed => self.player_window.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => {
                        std::process::exit(0);
                    }
                    Key::Up | Key::W => {
                        let player_input = self
                            .player
                            .get_component_mut::<CInput>(&mut self.entity_memory_pool)
                            .unwrap();

                        if !player_input.up.held {
                            player_input.up.pressed = true;
                            player_input.up.held = true;
                        }
                    }
                    Key::Down | Key::S => {
                        let player_input = self
                            .player
                            .get_component_mut::<CInput>(&mut self.entity_memory_pool)
                            .unwrap();

                        if !player_input.down.held {
                            player_input.down.pressed = true;
                            player_input.down.held = true;
                        }
                    }
                    Key::Left | Key::A => {
                        let player_input = self
                            .player
                            .get_component_mut::<CInput>(&mut self.entity_memory_pool)
                            .unwrap();

                        if !player_input.left.held {
                            player_input.left.pressed = true;
                            player_input.left.held = true;
                        }
                    }
                    Key::Right | Key::D => {
                        let player_input = self
                            .player
                            .get_component_mut::<CInput>(&mut self.entity_memory_pool)
                            .unwrap();

                        if !player_input.right.held {
                            player_input.right.pressed = true;
                            player_input.right.held = true;
                        }
                    }
                    Key::Space => {
                        let player_input = self
                            .player
                            .get_component_mut::<CInput>(&mut self.entity_memory_pool)
                            .unwrap();

                        if !player_input.space.held {
                            player_input.space.pressed = true;
                            player_input.space.held = true;
                        }
                    }
                    _ => {}
                },
                Event::KeyReleased { code, .. } => match code {
                    Key::Up | Key::W => {
                        let player_input = self
                            .player
                            .get_component_mut::<CInput>(&mut self.entity_memory_pool)
                            .unwrap();

                        player_input.up.held = false;
                        player_input.up.released = true;
                    }
                    Key::Down | Key::S => {
                        let player_input = self
                            .player
                            .get_component_mut::<CInput>(&mut self.entity_memory_pool)
                            .unwrap();

                        player_input.down.held = false;
                        player_input.down.released = true;
                    }
                    Key::Left | Key::A => {
                        let player_input = self
                            .player
                            .get_component_mut::<CInput>(&mut self.entity_memory_pool)
                            .unwrap();

                        player_input.left.held = false;
                        player_input.left.released = true;
                    }
                    Key::Right | Key::D => {
                        let player_input = self
                            .player
                            .get_component_mut::<CInput>(&mut self.entity_memory_pool)
                            .unwrap();

                        player_input.right.held = false;
                        player_input.right.released = true;
                    }
                    Key::Space => {
                        let player_input = self
                            .player
                            .get_component_mut::<CInput>(&mut self.entity_memory_pool)
                            .unwrap();

                        player_input.space.held = false;
                        player_input.space.released = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Player jump
        let (jump_input_release, input_x) = {
            let player_input = self
                .player
                .get_component_mut::<CInput>(&mut self.entity_memory_pool)
                .unwrap();

            let mut input_x = 0;

            if player_input.right.held {
                input_x += 1;
            }
            if player_input.left.held {
                input_x -= 1;
            }

            if player_input.space.pressed {
                self.jump_input_timer = self.jump_input_duration;
            }

            (player_input.space.released, input_x)
        };

        // Player physics
        let respawn_player = {
            let player_transform = self
                .player
                .get_component_mut::<CTransform>(&mut self.entity_memory_pool)
                .unwrap();

            // Gravity
            player_transform.acceleration.y = 9.81 * 0.25;

            // Jump
            if self.jump_input_timer > 0 {
                // Normal
                if self.grounded_timer > 0 {
                    player_transform.velocity.y = -30.0;
                    self.grounded_timer = 0;
                    self.jump_input_timer = 0;
                    self.jump_sfx.play();
                }
                // Wall jump
                else if self.wall_contact_timer != 0 {
                    player_transform.velocity.y = -40.0;
                    player_transform.velocity.x =
                        player_transform.max_speed * -self.wall_contact_timer.signum() as f32;

                    self.wall_contact_timer = 0;
                    self.grounded_timer = 0;
                    self.jump_input_timer = 0;
                    self.jump_sfx.play();
                    self.wall_jump_physics = true;
                }
            }
            // Start falling
            if jump_input_release && player_transform.velocity.y < 0.0 {
                player_transform.velocity.y /= 3.0;
            }

            // Move
            let desired_speed = input_x as f32 * player_transform.max_speed;

            // Different scaler for acceleration and deceleration
            let acceleration_scale = if input_x != 0 {
                self.player_acceleration_scale.x
            } else {
                self.player_acceleration_scale.y
            };

            if !self.wall_jump_physics {
                player_transform.acceleration.x =
                    (desired_speed - player_transform.velocity.x) * acceleration_scale;
            } else {
                player_transform.acceleration.x = 0.0;
                player_transform.velocity.x += input_x as f32 * 1.0;
                // Clamp velocity
                if player_transform.velocity.x.abs() > player_transform.max_speed {
                    player_transform.velocity.x =
                        player_transform.max_speed * player_transform.velocity.x.signum();
                }
            }

            // Update velocity and position
            player_transform.update();
            // Update window position
            self.player_window.set_position(Vector2::new(
                player_transform.position.x as i32 - self.player_window.size().x as i32 / 2,
                player_transform.position.y as i32 - self.player_window.size().y as i32 / 2,
            ));

            // Respawn player
            let respawn_player = if player_transform.position.y
                > *SCREEN_HEIGHT as f32 - player_transform.half_size.y
            {
                true
            } else {
                false
            };

            respawn_player
        };

        // Player dies
        if respawn_player {
            self.respawn_player();
            self.death_sfx.play();
        }

        // Update jump timers
        if self.jump_input_timer > 0 {
            self.jump_input_timer -= 1;
        }
        if self.grounded_timer > 0 {
            self.grounded_timer -= 1;
        }
        if self.wall_contact_timer != 0 {
            self.wall_contact_timer -= self.wall_contact_timer.signum();
            dbg!(self.wall_contact_timer);
        }
    }

    pub fn s_render(&mut self) {
        let platforms = self
            .entity_manager
            .get_entities_by_tag_mut(EntityTag::Platform)
            .unwrap();

        for platform in platforms.iter_mut() {
            let block_window = self.window_map.get_mut(&platform.id()).unwrap();

            block_window.clear(Color::BLACK);

            block_window.display();
        }

        self.goal_window.clear(Color::GREEN);
        self.goal_window.display();

        self.player_window.clear(Color::WHITE);
        self.player_window.display();

        self.player_window.request_focus();
    }

    pub fn s_collision(&mut self) {
        let (player_position, player_prev_position, player_half_size) = {
            let player_transform = self
                .player
                .get_component::<CTransform>(&mut self.entity_memory_pool)
                .unwrap();

            (
                player_transform.position,
                player_transform.prev_position,
                player_transform.half_size,
            )
        };

        let platforms = self
            .entity_manager
            .get_entities_by_tag_mut(EntityTag::Platform)
            .unwrap();

        let mut player_position_adjustment = Vector2::new(0.0, 0.0);

        for platform in platforms.iter_mut() {
            let platform_transform = platform
                .get_component_mut::<CTransform>(&mut self.entity_memory_pool)
                .unwrap();

            let overlap = platform_transform.get_overlap(player_position, player_half_size);

            if overlap.x > 0 && overlap.y > 0 {
                let prev_overlap =
                    platform_transform.get_overlap(player_prev_position, player_half_size);

                let adjustment_sign: Vector2<f32> = Vector2::new(
                    if platform_transform.position.x < player_position.x {
                        1.0
                    } else {
                        -1.0
                    },
                    if platform_transform.position.y < player_position.y {
                        1.0
                    } else {
                        -1.0
                    },
                );

                if prev_overlap.x > 0 {
                    player_position_adjustment.y = overlap.y as f32 * adjustment_sign.y;
                } else if prev_overlap.y > 0 {
                    player_position_adjustment.x = overlap.x as f32 * adjustment_sign.x;
                } else {
                    if overlap.x > overlap.y {
                        player_position_adjustment.y = overlap.y as f32 * adjustment_sign.y;
                    } else {
                        player_position_adjustment.x = overlap.x as f32 * adjustment_sign.x;
                    }
                }
            }
        }

        {
            let player_transform = self
                .player
                .get_component_mut::<CTransform>(&mut self.entity_memory_pool)
                .unwrap();

            player_transform.grounded = false;

            player_transform.position += player_position_adjustment;
            self.player_window.set_position(Vector2::new(
                player_transform.position.x as i32 - self.player_window.size().x as i32 / 2,
                player_transform.position.y as i32 - self.player_window.size().y as i32 / 2,
            ));

            if player_position_adjustment.y != 0.0 {
                if player_transform.velocity.y > 0.0 {
                    player_transform.grounded = true;
                    self.grounded_timer = self.grounded_duration;
                    self.wall_jump_physics = false;
                }
                player_transform.velocity.y = 0.0;
            }
            if player_position_adjustment.x != 0.0 {
                player_transform.velocity.x = 0.0;
                self.wall_contact_timer = self.wall_contact_duration as i32
                    * -player_position_adjustment.x.signum() as i32;
            }
        }

        let player_goal_overlap = self
            .goal
            .get_component_mut::<CTransform>(&mut self.entity_memory_pool)
            .as_mut()
            .unwrap()
            .get_overlap(player_position, player_half_size);

        if player_goal_overlap.x > 0 && player_goal_overlap.y > 0 {
            self.level_index += 1;

            self.load_level(self.level_index as u32);

            self.respawn_player();
            self.level_complete_sfx.play();
        }
    }

    pub fn respawn_player(&mut self) {
        let player_transform = self
            .player
            .get_component_mut::<CTransform>(&mut self.entity_memory_pool)
            .unwrap();

        player_transform.position = self.levels[self.level_index as usize].player_start;
        player_transform.prev_position = player_transform.position;
        player_transform.velocity = Vector2::new(0.0, 0.0);

        self.grounded_timer = 0;
        self.jump_input_timer = 0;
        self.wall_contact_timer = 0;
        self.wall_jump_physics = false;
    }

    pub fn load_level(&mut self, level_index: u32) {
        let level = &self.levels[level_index as usize];

        // Player
        {
            let player_transform = self
                .player
                .get_component_mut::<CTransform>(&mut self.entity_memory_pool)
                .unwrap();

            player_transform.position = level.player_start;
            player_transform.prev_position = level.player_start;
            player_transform.velocity = Vector2::new(0.0, 0.0);
        }

        // Goal
        {
            let goal_transform = self
                .goal
                .get_component_mut::<CTransform>(&mut self.entity_memory_pool)
                .unwrap();

            goal_transform.position = level.goal;
            goal_transform.prev_position = level.goal;
            goal_transform.velocity = Vector2::new(0.0, 0.0);

            self.goal_window.set_position(Vector2::new(
                goal_transform.position.x as i32 - goal_transform.half_size.x as i32,
                goal_transform.position.y as i32 - goal_transform.half_size.y as i32,
            ));
        }

        // Platforms

        // Destroy all platforms
        let platforms = self
            .entity_manager
            .get_entities_by_tag_mut(EntityTag::Platform)
            .unwrap();

        for platform in platforms.iter_mut() {
            platform.destroy(&mut self.entity_memory_pool);
        }
        self.entity_manager.update(&mut self.entity_memory_pool);

        // Clear the window map
        self.window_map.clear();

        // Create new platforms
        for platform_rect in level.platforms.iter() {
            let mut platform = self
                .entity_manager
                .add_entity(EntityTag::Platform, &mut self.entity_memory_pool);
            platform.add_component::<CTransform>(&mut self.entity_memory_pool);
            let platform_transform = platform
                .get_component_mut::<CTransform>(&mut self.entity_memory_pool)
                .unwrap();

            platform_transform.position = platform_rect.position;
            platform_transform.size = platform_rect.size;
            platform_transform.half_size = platform_transform.size / 2.0;

            let mut platform_window = RenderWindow::new(
                VideoMode::new(
                    platform_transform.size.x as u32,
                    platform_transform.size.y as u32,
                    32,
                ),
                "Platform",
                Style::NONE,
                &Default::default(),
            );
            platform_window.set_framerate_limit(FRAMERATE);
            platform_window.set_position(Vector2::new(
                platform_transform.position.x as i32 - platform_transform.half_size.x as i32,
                platform_transform.position.y as i32 - platform_transform.half_size.y as i32,
            ));

            self.window_map.insert(platform.id(), platform_window);
        }
    }
}

fn main() {
    let mut world = World::new();

    loop {
        world.s_update();
        world.s_collision();
        world.s_render();
    }
}
