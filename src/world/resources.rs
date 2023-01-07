use super::prelude::*;

#[derive(Resource, Default, Clone, Copy, Debug)]
pub enum Weather {
    #[default]
    Clear = 0,
    Drizzle = 1,
    Rain = 3,
    HeavyRain = 4,
}

impl Weather {
    /// Returns `true` if the weather is [`Clear`].
    ///
    /// [`Clear`]: Weather::Clear
    #[must_use]
    pub fn is_clear(&self) -> bool {
        matches!(self, Self::Clear)
    }

    /// Mutates weather to previous intensity level.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let mut weather = Weather::Drizzle;
    ///
    /// weather.lessen();
    ///
    /// assert_eq!(result, Weather::Clear);
    /// ```
    pub fn lessen(&mut self) {
        *self = match self {
            Weather::HeavyRain => Weather::Rain,
            Weather::Rain => Weather::Drizzle,
            Weather::Drizzle => Weather::Clear,
            Weather::Clear => Weather::Clear,
        };
    }

    /// Mutates weather to next intensity level.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let mut weather = Weather::Clear;
    ///
    /// weather.intensify();
    ///
    /// assert_eq!(result, Weather::Drizzle);
    /// ```
    pub fn intensify(&mut self) {
        *self = match self {
            Weather::Clear => Weather::Drizzle,
            Weather::Drizzle => Weather::Rain,
            Weather::Rain => Weather::HeavyRain,
            Weather::HeavyRain => Weather::HeavyRain,
        }
    }
}
