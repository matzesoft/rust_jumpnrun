use bevy::prelude::Bundle;
use bevy_rapier2d::dynamics::{LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, ColliderMassProperties, Friction};

/// Component for ghost player entity
///
/// # Fields
///
/// * `collider` - The collider for the ghost player entity.
/// * `rigid_body` - The rigid body for the ghost player entity.
/// * `velocity` - The velocity of the ghost player entity.
/// * `rotation_constraints` - The rotation constraints for the ghost player entity.
/// * `friction` - The friction of the ghost player entity.
/// * `density` - The density of the ghost player entity.
#[derive(Clone, Default, Bundle)]
pub struct GhostColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}
