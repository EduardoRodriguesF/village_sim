mod headless_transform;

mod prelude {
    pub use crate::headless_transform::components::*;
    pub use bevy::prelude::*;

    pub const SCALE: f32 = 1.;
}

use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HeadlessTransform)
        .run();
}
