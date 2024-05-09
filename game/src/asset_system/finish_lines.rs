use std::collections::{HashMap, HashSet};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::{ActiveEvents, Collider, Friction, Sensor};
use bevy_rapier2d::pipeline::CollisionEvent;

#[derive(Default, Component)]
pub struct FinishLine;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct FinishLineBundle {
    finishline: FinishLine,
}

pub fn spawn_finishline_collision(
    mut commands: Commands,
    finishline_query: Query<(&GridCoords, &Parent), Added<FinishLine>>,
    parent_query: Query<&Parent, Without<FinishLine>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    /// Represents a wide finishline that is 1 tile tall
    /// Used to spawn finishline collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a finishline of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the finishlines are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the finishline belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the finishlines to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_finishline_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    finishline_query.iter().for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_finishline_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });

    if !finishline_query.is_empty() {
        level_query.iter().for_each(|(level_entity, level_iid)| {
            if let Some(level_finishlines) = level_to_finishline_locations.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level.layer_instances()[0];

                // combine finishline tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        match (plate_start, level_finishlines.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut finishline_rects: Vec<Rect> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                finishline_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for finishline_rect in finishline_rects {
                        level
                            .spawn_empty()
                            .insert(Collider::cuboid(
                                (finishline_rect.right as f32 - finishline_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (finishline_rect.top as f32 - finishline_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ))
                            .insert(RigidBody::Fixed)
                            .insert(Friction::new(1.0))
                            .insert(Transform::from_xyz(
                                (finishline_rect.left + finishline_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (finishline_rect.bottom + finishline_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default())
                            .insert(FinishLine);
                    }
                });
            }
        });
    }
}

#[derive(Clone, Default, Component)]
pub struct FinishLineDetection {
    pub on_finishline: bool,
}

#[derive(Component)]
pub struct FinishLineSensor {
    pub finishline_detection_entity: Entity,
    pub intersecting_finishline_entities: HashSet<Entity>,
}

pub fn spawn_finishline_sensor(
    mut commands: Commands,
    detect_finishline_for: Query<(Entity, &Collider), Added<FinishLineDetection>>,
) {
    for (entity, shape) in &detect_finishline_for {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 {
                x: half_extents_x,
                y: half_extents_y,
            } = cuboid.half_extents();

            let detector_shape = Collider::cuboid(half_extents_x  * 1.05, half_extents_y * 1.05);

            let sensor_translation = Vec3::new(0., 0., 0.);

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn_empty()
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(detector_shape)
                    .insert(Sensor)
                    .insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default())
                    .insert(FinishLineSensor {
                        finishline_detection_entity: entity,
                        intersecting_finishline_entities: HashSet::new(),
                    });
            });
        }
    }
}

pub fn finishline_detection(
    mut finishline_sensors: Query<&mut FinishLineSensor>,
    mut collisions: EventReader<CollisionEvent>,
    collidables: Query<Entity, (With<Collider>, Without<Sensor>)>,
    finishlines: Query<Entity, With<FinishLine>>
) {
    for collision_event in collisions.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if collidables.contains(*e1) && finishlines.contains(*e1) {
                    if let Ok(mut sensor) = finishline_sensors.get_mut(*e2) {
                        sensor.intersecting_finishline_entities.insert(*e1);
                    }
                } else if collidables.contains(*e2) && finishlines.contains(*e2) {
                    if let Ok(mut sensor) = finishline_sensors.get_mut(*e1) {
                        sensor.intersecting_finishline_entities.insert(*e2);
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if collidables.contains(*e1) && finishlines.contains(*e1) {
                    if let Ok(mut sensor) = finishline_sensors.get_mut(*e2) {
                        sensor.intersecting_finishline_entities.remove(e1);
                    }
                } else if collidables.contains(*e2) && finishlines.contains(*e2) {
                    if let Ok(mut sensor) = finishline_sensors.get_mut(*e1) {
                        sensor.intersecting_finishline_entities.remove(e2);
                    }
                }
            }
        }
    }
}

pub fn update_on_finishline(
    mut finishline_detectors: Query<&mut FinishLineDetection>,
    finishline_sensors: Query<&FinishLineSensor, Changed<FinishLineSensor>>,
    mut transforms: Query<&mut Transform>,
) {
    for sensor in &finishline_sensors {
        if let Ok(mut finishline_detection) = finishline_detectors.get_mut(sensor.finishline_detection_entity) {
            finishline_detection.on_finishline = !sensor.intersecting_finishline_entities.is_empty();
            if finishline_detection.on_finishline {
                if let Ok(mut transform) = transforms.get_mut(sensor.finishline_detection_entity) {
                    // Set the new position for the entity
                    transform.translation = Vec2::new(40., 40.).extend(0.0);
                }
            }
        }
    }
}