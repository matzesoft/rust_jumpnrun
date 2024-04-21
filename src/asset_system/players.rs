use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::asset_system::collision::ColliderBundle;

use crate::input_system;
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    //pub velocity: Velocity,
    //pub rigid_body: RigidBody,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,

    pub collider_bundle: ColliderBundle,

    pub input_handler: input_system::input_handler::InputHandler,
}
