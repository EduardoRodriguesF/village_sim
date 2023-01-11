use crate::prelude::*;
use bevy::input::mouse::*;

pub fn manual_scale_change(
    mut scroll_evr: EventReader<MouseWheel>,
    mut target_scale: ResMut<TargetScale>,
) {
    for ev in scroll_evr.iter() {
        let delta = ev.y / 10.;

        target_scale.value += delta;
    }
}

pub fn notify_scale_change(scale: Res<PixelScale>) {
    if scale.is_changed() {
        info!("Scale: {:?}", scale.0);
    }
}
