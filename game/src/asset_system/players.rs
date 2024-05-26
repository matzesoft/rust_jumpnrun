use crate::asset_system::collision::ColliderBundle;
use crate::asset_system::ghost_physics::GhostColliderBundle;
use crate::asset_system::walls::GroundDetection;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::asset_system::finish_lines::FinishLineDetection;
use crate::asset_system::traps::TrapDetection;

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
    pub trap_detection: TrapDetection,
    pub finishline_detection: FinishLineDetection,
    pub input_handler: input_system::input_handler::InputHandler,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct GhostPlayer {
    pub id: u64,
}
#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct GhostPlayerBundle {
    pub ghost_player: GhostPlayer,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub ghost_collider_bundle: GhostColliderBundle,
}
