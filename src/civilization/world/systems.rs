use crate::npc::prelude::*;

pub fn spawn_entities(mut commands: Commands, map: Res<Map>) {
    for entity in map.entities.iter() {
        let area = Rect::new(0., 0., entity.width as f32, entity.height as f32);

        let maybe_entity_commands = match entity.identifier.as_str() {
            "MarketTent" => Some(commands.spawn((
                Activity {
                    avg_time_in_seconds: 12.,
                    area,
                },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::LIME_GREEN,
                        rect: Some(Rect::new(0., 0., entity.width as f32, entity.height as f32)),
                        anchor: bevy::sprite::Anchor::BottomLeft,
                        ..default()
                    },
                    ..default()
                },
            ))),
            "Entrance" => Some(commands.spawn((
                Activity {
                    avg_time_in_seconds: 0.15,
                    area,
                },
                Entrance,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::PINK,
                        rect: Some(Rect::new(0., 0., entity.width as f32, entity.height as f32)),
                        anchor: bevy::sprite::Anchor::BottomLeft,
                        ..default()
                    },
                    ..default()
                },
            ))),
            _ => None,
        };

        if let Some(mut entity_commands) = maybe_entity_commands {
            let pos = entity.position.extend(1.);
            let transform = HeadlessTransform(Transform::from_translation(pos));

            entity_commands.insert((Identifier(entity.identifier.to_owned()), transform));
        }
    }
}

pub fn populate(
    mut commands: Commands,
    mut seed: ResMut<Seed>,
    max_population: Res<MaxPopulation>,
    q_npcs: Query<With<NpcStats>>,
    q_entrances: Query<(&HeadlessTransform, &Activity), With<Entrance>>,
) {
    const PADDING: f32 = 4.;
    let population = q_npcs.iter().len() as u16;
    let entrances = q_entrances
        .iter()
        .collect::<Vec<(&HeadlessTransform, &Activity)>>();

    if population.cmp(&max_population).is_ge() {
        return;
    }

    for _ in population..**max_population {
        let r: f32 = seed.rng.gen();
        let g: f32 = seed.rng.gen();
        let b: f32 = seed.rng.gen();

        let (pos, entrance) = entrances
            .get(seed.rng.gen_range(0..entrances.len()))
            .unwrap();

        let pos = pos.0.translation
            + Vec3::new(
                seed.rng.gen_range(PADDING..entrance.area.width() - PADDING),
                seed.rng.gen_range(PADDING..entrance.area.height() - PADDING),
                0.,
            );

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(r, g, b),
                    rect: Some(Rect::new(0., 0., 16., 32.)),
                    anchor: bevy::sprite::Anchor::BottomCenter,
                    ..default()
                },
                ..default()
            },
            NpcStats::new(&mut seed.rng),
            Routine {
                activities: vec![RoutineItem::from_search("MarketTent"), RoutineItem::exit()],
                is_loop: true,
                ..default()
            },
            HeadlessTransform(Transform::from_translation(pos)),
            Velocity::new(0., 0.),
            Collider::new(Vec2::new(6., 6.), bevy::sprite::Anchor::BottomCenter),
        ));
    }
}

pub fn create_walls(mut commands: Commands, map: Res<Map>) {
    for (row_idx, row) in map.data.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            if tile.is_none() {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::hex("333").unwrap(),
                            rect: Some(Rect::new(0., 0., 16., 16.)),
                            anchor: bevy::sprite::Anchor::BottomLeft,
                            ..default()
                        },
                        ..default()
                    },
                    HeadlessTransform(Transform::from_xyz(
                        col_idx as f32 * 16.,
                        row_idx as f32 * 16.,
                        1.,
                    )),
                    Collider::new(Vec2::splat(16.), bevy::sprite::Anchor::Center),
                ));
            }
        }
    }
}
