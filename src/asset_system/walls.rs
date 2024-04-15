use std::collections::HashSet;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::asset_system::assets_loading::GRID_SIZE;


#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Default, Resource)]
pub struct LevelWalls {
    wall_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}