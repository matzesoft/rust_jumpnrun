use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::asset_system::assets_loading::GRID_SIZE;

#[derive(Default, Component)]
struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}