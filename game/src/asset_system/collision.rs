use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

/// Bundle for player collider
///
/// This bundle is used to create a collider for the player entity.
///
/// # Fields
///
/// * `collider` - The collider for the player entity.
/// * `rigid_body` - The rigid lets the game know that this body can be affected by external sources.
/// * `velocity` - The velocity of the player entity.
/// * `rotation_constraints` - The rotation constraints stop the player from rolling like a ball.
/// * `gravity_scale` - The gravity scale of the player entity.
/// * `friction` - The friction of the player entity adds resistance when moving on another object.
/// * `density` - The density of the player entity, lets the player have density making it more realistic.
#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

/// Creates an instance of the `ColliderBundle` struct for every player entity.
///
/// can be easily expanded to include more entities using the match case.
///
/// # Arguments
///
/// * `entity_instance` - A reference to an `EntityInstance` struct like a player.
impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(6.5, 8.0),
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}