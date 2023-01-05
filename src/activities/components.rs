use super::prelude::*;

#[derive(Component, Clone, PartialEq, Eq, DerefMut, Deref)]
pub struct Identifier(pub String);

#[derive(Component)]
pub struct Activity {
    pub avg_time_in_seconds: f32,
    pub area: Rect,
}

impl Default for Activity {
    fn default() -> Self {
        Activity {
            avg_time_in_seconds: 6.,
            area: Rect::new(0., 0., 16., 16.),
        }
    }
}

#[derive(Component)]
pub struct ActivityPlan {
    pub activity: Entity,
}

#[derive(Component, Deref, DerefMut)]
pub struct SearchingActivity(pub Identifier);

#[derive(Component, Clone)]
pub struct Busy {
    pub location: Option<Vec2>,
    pub timer: Timer,
}

impl Busy {
    pub fn from_location(location: Vec2, secs: f32) -> Self {
        let timer = Timer::from_seconds(secs, TimerMode::Once);

        Self {
            timer,
            location: Some(location),
        }
    }
}

#[derive(Component, Default)]
pub struct RoutineItem {
    pub activity: Option<Entity>,
    pub search: Option<String>,
}

impl RoutineItem {
    pub fn from_search(identifier: &str) -> RoutineItem {
        RoutineItem {
            search: Some(identifier.to_string()),
            ..default()
        }
    }

    pub fn exit() -> RoutineItem {
        RoutineItem {
            search: Some("Entrance".to_string()),
            ..default()
        }
    }
}

#[derive(Component, Default)]
pub struct Routine {
    pub current_idx: Option<usize>,
    pub activities: Vec<RoutineItem>,
    pub is_loop: bool,
}

impl Routine {
    pub fn get_current(&self) -> Option<&RoutineItem> {
        match self.current_idx {
            Some(idx) => self.activities.get(idx),
            None => None,
        }
    }

    pub fn next(&mut self) -> Option<&RoutineItem> {
        let mut idx = match self.current_idx {
            Some(v) => v + 1,
            None => 0,
        };

        if idx >= self.activities.len() && self.is_loop {
            idx = 0;
        }

        self.current_idx = Some(idx);
        self.get_current()
    }
}

#[derive(Component, Default)]
pub struct Reaction {
    pub timer: Timer,
}
