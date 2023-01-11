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

#[derive(Resource, Debug)]
pub struct TargetScale {
    pub value: f32,
    pub speed: f32,
}

impl Default for TargetScale {
    fn default() -> Self {
        TargetScale {
            value: PixelScale::default().0,
            speed: 0.025,
        }
    }
}
