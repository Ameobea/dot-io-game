//! Responsible for generating the initial version of the world.

use std::f32::consts::PI;

use nalgebra::{Isometry2, Point2, Vector2};
use nphysics2d::algebra::Velocity2;
use rand::{thread_rng, Rng};

// use conf::CONF;
use physics::entities::{AsteroidEntity, Entity, EntitySpawn};

#[inline(always)]
fn pt2(x: f32, y: f32) -> Point2<f32> {
    Point2::new(x, y)
}

fn get_asteroid_vertices() -> Vec<Point2<f32>> {
    [
        pt2(-1., 1.),
        pt2(-1., -1.),
        pt2(1., -1.),
        pt2(2., 0.),
        pt2(1., 1.),
    ]
        .into_iter()
        .map(|pt| pt * 20.)
        .collect()
}

fn create_asteroid() -> EntitySpawn {
    let mut rng = thread_rng();

    EntitySpawn {
        isometry: Isometry2::new(
            Vector2::new(
                // rng.gen_range(CONF.game.world_min_x, CONF.game.world_max_x),
                // rng.gen_range(CONF.game.world_min_y, CONF.game.world_max_y),
                rng.gen_range(50., 500.),
                rng.gen_range(50., 500.),
            ),
            rng.gen_range(0., 2.0 * PI),
        ),
        entity: Entity::Asteroid(AsteroidEntity {
            vertices: get_asteroid_vertices(),
        }),
        velocity: Velocity2::new(
            Vector2::new(rng.gen_range(0.0, 0.05), rng.gen_range(0.0, 0.05)),
            rng.gen_range(-0.025, 0.025),
        ),
        data: (),
    }
}

pub fn get_initial_entities() -> Vec<EntitySpawn> {
    vec![create_asteroid(), create_asteroid()]
}
