pub mod prelude;

use bevy::prelude::*;

pub struct SafeDespawnPlugin;

impl Plugin for SafeDespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(clean.in_base_set(CoreSet::Last));
    }
}

#[derive(Component)]
pub struct ScheduledDespawn;

pub fn clean(mut commands: Commands, query: Query<Entity, Added<ScheduledDespawn>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
