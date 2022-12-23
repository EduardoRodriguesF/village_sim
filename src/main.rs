mod prelude {
    pub use bevy::prelude::*;
}

use prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
