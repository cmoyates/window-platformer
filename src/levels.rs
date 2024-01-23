use sfml::system::Vector2;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct PlatformRect {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
}

pub struct Level {
    pub platforms: Vec<PlatformRect>,
    pub player_start: Vector2<f32>,
    pub goal: Vector2<f32>,
}

pub fn get_levels() -> Vec<Level> {
    let level_1 = Level {
        platforms: vec![
            PlatformRect {
                position: Vector2::new(*SCREEN_WIDTH as f32 / 2.0, *SCREEN_HEIGHT as f32 * 0.75),
                size: Vector2::new(1000.0, 50.0),
            },
            PlatformRect {
                position: Vector2::new(
                    *SCREEN_WIDTH as f32 / 2.0,
                    *SCREEN_HEIGHT as f32 * 0.5 + 200.0,
                ),
                size: Vector2::new(250.0, 100.0),
            },
            // PlatformRect {
            //     position: Vector2::new(*SCREEN_WIDTH as f32 * 0.25, *SCREEN_HEIGHT as f32 * 0.5),
            //     size: Vector2::new(50.0, 500.0),
            // },
            // PlatformRect {
            //     position: Vector2::new(*SCREEN_WIDTH as f32 * 0.75, *SCREEN_HEIGHT as f32 * 0.5),
            //     size: Vector2::new(50.0, 500.0),
            // },
        ],
        player_start: Vector2::new(
            *SCREEN_WIDTH as f32 / 4.0 + 100.0,
            *SCREEN_HEIGHT as f32 * 0.75 - 75.0,
        ),
        goal: Vector2::new(
            *SCREEN_WIDTH as f32 * 3.0 / 4.0 - 100.0,
            *SCREEN_HEIGHT as f32 * 0.75 - 75.0,
        ),
    };

    let level_2 = Level {
        platforms: vec![
            PlatformRect {
                position: Vector2::new(*SCREEN_WIDTH as f32 / 2.0, *SCREEN_HEIGHT as f32 * 0.75),
                size: Vector2::new(1000.0, 50.0),
            },
            PlatformRect {
                position: Vector2::new(*SCREEN_WIDTH as f32 * 0.65, *SCREEN_HEIGHT as f32 * 0.25),
                size: Vector2::new(400.0, 50.0),
            },
            PlatformRect {
                position: Vector2::new(*SCREEN_WIDTH as f32 * 0.45, *SCREEN_HEIGHT as f32 * 0.2),
                size: Vector2::new(50.0, 800.0),
            },
            PlatformRect {
                position: Vector2::new(*SCREEN_WIDTH as f32 * 0.55, *SCREEN_HEIGHT as f32 * 0.5),
                size: Vector2::new(50.0, 500.0),
            },
            // PlatformRect {
            //     position: Vector2::new(*SCREEN_WIDTH as f32 * 0.75, *SCREEN_HEIGHT as f32 * 0.5),
            //     size: Vector2::new(50.0, 500.0),
            // },
        ],
        player_start: Vector2::new(
            *SCREEN_WIDTH as f32 / 4.0 + 100.0,
            *SCREEN_HEIGHT as f32 * 0.75 - 75.0,
        ),
        goal: Vector2::new(
            *SCREEN_WIDTH as f32 * 3.0 / 4.0 - 100.0,
            *SCREEN_HEIGHT as f32 * 0.25 - 75.0,
        ),
    };

    let levels = vec![level_1, level_2];

    return levels;
}
