use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::asset_system::collision::ColliderBundle;
use crate::asset_system::ground::GroundDetection;

use crate::input_system;
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub ground_detection: GroundDetection,
    pub input_handler: input_system::input_handler::InputHandler,
}