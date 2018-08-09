//! Every "thing" in the game is an entity.  Every entity is renderable, and the game loop runs
//! by looping over all entities and rendering them.

use nalgebra::{Isometry2, Point2};
use ncollide2d::bounding_volume::aabb::AABB;
use ncollide2d::shape::Shape;

use proto_utils::ServerMessageContent;
use protos::server_messages::MovementUpdate;

pub trait Entity: Shape<f32> {
    fn render(&self, tick: usize);

    /// Updates this entity's state for one tick.  Returns `true` if the entity has moved or
    /// changed shape in a way that it has a new `BoundingVolume`.
    fn tick(&mut self, tick: usize) -> bool;

    /// Updates the entity's state with the data from a message from the server.  REturns `true` if
    /// the entity moved or changed shape as a result of the update in a way that it has a new
    /// `BoundingVolume`.
    fn apply_update(&mut self, update: &ServerMessageContent) -> bool;

    fn get_bounding_volume(&self) -> AABB<f32>;

    fn get_isometry(&self) -> &Isometry2<f32>;

    fn get_vertices(&self) -> &[Point2<f32>];

    fn set_movement(&mut self, &MovementUpdate);
}
