//! Contains implementation-specific code that is not generic for the engine.

use super::render_quad;
use conf::CONF;
use entity::Entity;
use game_state::get_effects_manager;
use protos::message_common::MovementDirection as Direction;
use protos::server_messages::{
    MovementUpdate, ServerMessage_oneof_payload as ServerMessageContent,
};
use util::{error, log, math_random, Color};

pub mod effects;

use self::effects::DemoCircleEffect;

struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb { red, green, blue }
    }
}

/// The basic entity that is used right now.  They're all rendered as a square, but they all have
/// a unique color.
pub struct PlayerEntity {
    color: Rgb,
    pub pos_x: f32,
    pub pos_y: f32,
    /// Which way the user is telling this entity to go
    pub direction_input: Direction,
    /// Speed along the x axis in pixels/tick
    pub velocity_x: f32,
    /// Speed along the y axis in pixels/tick
    pub velocity_y: f32,
    pub size: u16,
}

impl PlayerEntity {
    pub fn new(pos_x: f32, pos_y: f32, size: u16) -> Self {
        PlayerEntity {
            color: Rgb::new(
                (math_random() * 255.) as u8,
                (math_random() * 255.) as u8,
                (math_random() * 255.) as u8,
            ),
            pos_x,
            pos_y,
            direction_input: Direction::STOP,
            velocity_x: 0.,
            velocity_y: 0.,
            size,
        }
    }

    fn set_movement(
        &mut self,
        &MovementUpdate {
            pos_x,
            pos_y,
            velocity_x,
            velocity_y,
            ..
        }: &MovementUpdate,
    ) {
        log(format!(
            "{}, {}, {}, {}",
            pos_x, pos_y, velocity_x, velocity_y
        ));
        self.pos_x = pos_x;
        self.pos_y = pos_y;
        self.velocity_x = velocity_x;
        self.velocity_y = velocity_y;
    }

    fn tick_movement(&mut self) {
        let (x_diff, y_diff) = match self.direction_input {
            Direction::DOWN => (0., 1.),
            Direction::DOWN_LEFT => (-1., 1.),
            Direction::DOWN_RIGHT => (1., 1.),
            Direction::LEFT => (-1., 0.),
            Direction::RIGHT => (1., 0.),
            Direction::STOP => (0., 0.),
            Direction::UP => (0., -1.),
            Direction::UP_LEFT => (-1., -1.),
            Direction::UP_RIGHT => (1., -1.),
        };

        let acceleration = CONF.physics.acceleration_per_tick;
        let max_speed = CONF.physics.max_player_speed;
        let (x_diff, y_diff) = (x_diff * acceleration, y_diff * acceleration);
        if x_diff + y_diff < max_speed {
            self.velocity_x += x_diff;
            self.velocity_y += y_diff;
        }

        self.pos_x += self.velocity_x;
        self.pos_y += self.velocity_y;
        self.pos_x *= 1. - CONF.physics.friction_per_tick;
        self.pos_y *= 1. - CONF.physics.friction_per_tick;
    }
}

impl Entity for PlayerEntity {
    fn render(&self) {
        render_quad(
            self.color.red,
            self.color.green,
            self.color.blue,
            self.pos_x as u16,
            self.pos_y as u16,
            self.size,
            self.size,
        );
    }

    fn tick(&mut self, tick: usize) {
        self.tick_movement();

        if tick % 120 == 0 {
            let effect = DemoCircleEffect {
                color: Color::random(),
                width: 3,
                x: self.pos_x as f32,
                y: (self.pos_y + 50.) as f32,
                cur_size: 0.,
                max_size: 50.,
                increment: 0.5,
            };

            get_effects_manager().add_effect(box effect);
        }
    }

    fn apply_update(&mut self, update: &ServerMessageContent) {
        match update {
            ServerMessageContent::movement_update(movement_update) => {
                self.set_movement(movement_update);
            }
            _ => error("Unexpected server message type received in entity update handler!"),
        }
    }
}
