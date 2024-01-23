use std::any::Any;

use sfml::system::Vector2;

pub trait Component: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn is_active(&self) -> bool;

    fn set_active(&mut self, active: bool);

    fn reset(&mut self);
}

#[derive(Clone, Debug)]
pub struct CTransform {
    active: bool,
    pub position: Vector2<f32>,
    pub prev_position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub size: Vector2<f32>,
    pub half_size: Vector2<f32>,
    pub max_speed: f32,
    pub scale: f32,
    pub grounded: bool,
}

impl CTransform {
    pub fn new() -> Self {
        Self {
            active: false,
            position: Vector2::new(0.0, 0.0),
            prev_position: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            size: Vector2::new(0.0, 0.0),
            half_size: Vector2::new(0.0, 0.0),
            max_speed: 4.0,
            scale: 1.0,
            grounded: false,
        }
    }

    pub fn update(&mut self) {
        self.prev_position = self.position;

        self.velocity += self.acceleration;
        self.position += self.velocity;
    }

    pub fn get_overlap(
        &mut self,
        other_position: Vector2<f32>,
        other_half_size: Vector2<f32>,
    ) -> Vector2<i32> {
        let delta: Vector2<i32> = Vector2::new(
            ((self.position.x - other_position.x) as i32).abs(),
            ((self.position.y - other_position.y) as i32).abs(),
        );

        let overlap: Vector2<i32> = Vector2::new(
            (self.half_size.x + other_half_size.x) as i32 - delta.x,
            (self.half_size.y + other_half_size.y) as i32 - delta.y,
        );

        return overlap;
    }
}

impl Component for CTransform {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn reset(&mut self) {
        self.active = true;
        self.position.x = 0.0;
        self.position.y = 0.0;
        self.prev_position.x = 0.0;
        self.prev_position.y = 0.0;
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
        self.acceleration.x = 0.0;
        self.acceleration.y = 0.0;
        self.size.x = 0.0;
        self.size.y = 0.0;
        self.half_size.x = 0.0;
        self.half_size.y = 0.0;
        self.max_speed = 0.0;
        self.scale = 1.0;
    }
}

#[derive(Clone)]
pub struct CLifetime {
    active: bool,
    pub lifetime_duration: f32,
    pub lifetime_timer: f32,
}

impl CLifetime {
    pub fn new(lifetime: f32) -> Self {
        Self {
            active: false,
            lifetime_duration: lifetime,
            lifetime_timer: lifetime,
        }
    }

    pub fn get_percentage_elapsed(&self) -> f32 {
        1.0 - self.lifetime_timer / self.lifetime_duration
    }

    pub fn get_percentage_remaining(&self) -> f32 {
        self.lifetime_timer / self.lifetime_duration
    }
}

impl Component for CLifetime {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn reset(&mut self) {
        self.active = true;
        self.lifetime_duration = 0.0;
        self.lifetime_timer = 0.0;
    }
}

#[derive(Clone, Debug)]
pub struct ButtonState {
    pub pressed: bool,
    pub held: bool,
    pub released: bool,
}

#[derive(Clone, Debug)]
pub struct CInput {
    active: bool,
    pub up: ButtonState,
    pub down: ButtonState,
    pub left: ButtonState,
    pub right: ButtonState,
    pub space: ButtonState,
}

impl CInput {
    pub fn new() -> Self {
        Self {
            active: false,
            up: ButtonState {
                pressed: false,
                held: false,
                released: false,
            },
            down: ButtonState {
                pressed: false,
                held: false,
                released: false,
            },
            left: ButtonState {
                pressed: false,
                held: false,
                released: false,
            },
            right: ButtonState {
                pressed: false,
                held: false,
                released: false,
            },
            space: ButtonState {
                pressed: false,
                held: false,
                released: false,
            },
        }
    }
}

impl Component for CInput {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn reset(&mut self) {
        self.active = true;
        self.up.pressed = false;
        self.up.held = false;
        self.up.released = false;
        self.down.pressed = false;
        self.down.held = false;
        self.down.released = false;
        self.left.pressed = false;
        self.left.held = false;
        self.left.released = false;
        self.right.pressed = false;
        self.right.held = false;
        self.right.released = false;
        self.space.pressed = false;
        self.space.held = false;
        self.space.released = false;
    }
}

#[derive(Clone, Debug)]
pub struct CAIBasic {
    active: bool,
}

impl CAIBasic {
    pub fn new() -> Self {
        Self { active: false }
    }
}

impl Component for CAIBasic {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn reset(&mut self) {
        self.active = true;
    }
}

#[derive(Clone, Debug)]
pub struct CHealth {
    active: bool,

    pub health: f32,
    pub max_health: f32,
}

impl CHealth {
    pub fn new(health: f32) -> Self {
        Self {
            active: false,
            health,
            max_health: health,
        }
    }
}

impl Component for CHealth {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn reset(&mut self) {
        self.active = true;
        self.health = 0.0;
        self.max_health = 0.0;
    }
}

#[derive(Clone, Debug)]
pub struct CDamage {
    active: bool,
    pub damage: f32,
}

impl CDamage {
    pub fn new(damage: f32) -> Self {
        Self {
            active: false,
            damage,
        }
    }
}

impl Component for CDamage {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn reset(&mut self) {
        self.active = true;
        self.damage = 0.0;
    }
}
