use bevy::prelude::*;

#[derive(
    Component, Debug, PartialEq, Clone, Copy, Reflect, FromReflect, Default, Deref, DerefMut,
)]
pub struct HeadlessTransform(pub Transform);
