use bevy::prelude::*;

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct PixelScale(f32);

impl Default for PixelScale {
    fn default() -> Self {
        Self(1.)
    }
}
