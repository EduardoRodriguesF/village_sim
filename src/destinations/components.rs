use super::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct DestinationNode(pub MapNode);

#[derive(Component, Deref, DerefMut)]
pub struct InstructionsToDestination(pub Vec<MapNode>);
