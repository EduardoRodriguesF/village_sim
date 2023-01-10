use bevy::prelude::*;

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct PixelScale(pub f32);

impl PixelScale {
    pub fn splat(&self) -> Vec2 {
        Vec2::splat(**self)
    }

    pub fn extended_splat(&self) -> Vec3 {
        self.splat().extend(1.)
    }
}

impl Default for PixelScale {
    fn default() -> Self {
        Self(1.)
    }
}
