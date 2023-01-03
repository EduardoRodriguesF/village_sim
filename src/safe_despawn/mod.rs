use bevy::prelude::*;

pub struct SafeDespawnPlugin;

impl Plugin for SafeDespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Last, clean);
    }
}

#[derive(Component)]
pub struct ScheduledDespawn;

pub fn clean(mut commands: Commands, query: Query<Entity, Added<ScheduledDespawn>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
