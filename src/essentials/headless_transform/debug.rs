use crate::prelude::*;
use bevy::input::mouse::*;

pub fn manual_scale_change(mut scroll_evr: EventReader<MouseWheel>, mut scale: ResMut<PixelScale>) {
    for ev in scroll_evr.iter() {
        let delta = ev.y / 10.;

        scale.0 += delta;
    }
}
