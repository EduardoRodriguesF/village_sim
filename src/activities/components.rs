use super::prelude::*;

#[derive(Component)]
pub struct Activity {
    pub avg_time_in_seconds: f32,
}

impl Default for Activity {
    fn default() -> Self {
        Activity {
            avg_time_in_seconds: 6.,
        }
    }
}

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

#[derive(Component)]
pub struct Routine {
    pub current_idx: Option<usize>,
    pub activities: Vec<Busy>,
    pub is_loop: bool,
}

impl Routine {
    pub fn get_current(&self) -> Option<&Busy> {
        match self.current_idx {
            Some(idx) => self.activities.get(idx),
            None => None,
        }
    }

    pub fn next(&mut self) -> Option<&Busy> {
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

impl Default for Routine {
    fn default() -> Self {
        Self {
            current_idx: None,
            activities: Vec::new(),
            is_loop: false,
        }
    }
}
