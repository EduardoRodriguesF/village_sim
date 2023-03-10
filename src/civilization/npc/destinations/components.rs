use super::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct DestinationPoint(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct InstructionsToDestination(pub Vec<Vec2>);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ReconsiderPath;
