use bevy::prelude::Bundle;
use bevy_ecs_ldtk::LdtkIntCell;
use bevy_rapier2d::dynamics::{GravityScale, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, ColliderMassProperties, Friction};

#[derive(Clone, Default, Bundle)]
pub struct GhostColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}
