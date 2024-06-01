use crate::asset_system::collision::ColliderBundle;
use crate::asset_system::ghost_physics::GhostColliderBundle;
use crate::asset_system::walls::GroundDetection;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::asset_system::finish_lines::FinishLineDetection;
use crate::asset_system::traps::TrapDetection;

use crate::input_system;

/// Component for player entity
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

/// Bundle for player entity
///
/// # Fields
///
/// * `player` - The player entity.
/// * `sprite_sheet_bundle` - The sprite sheet bundle for the player entity, comes from the ldtk File.
/// * `grid_coords` - The grid coordinates of the player entity.
/// * `collider_bundle` - The collider bundle for the player entity.
/// * `ground_detection` - The ground detection for the player entity.
/// * `trap_detection` - The trap detection for the player entity.
/// * `finishline_detection` - The finish line detection for the player entity.
/// * `input_handler` - The input handler for the player entity.
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

/// Component for ghost player entity
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct GhostPlayer {
    pub id: u64,
}

/// Bundle for ghost player entity
///
/// # Fields
///
/// * `ghost_player` - The ghost player entity.
/// * `sprite_sheet_bundle` - The sprite sheet bundle for the ghost player entity.
/// * `ghost_collider_bundle` - The collider bundle for the ghost player entity.
#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct GhostPlayerBundle {
    pub ghost_player: GhostPlayer,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub ghost_collider_bundle: GhostColliderBundle,
}
