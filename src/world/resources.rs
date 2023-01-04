use super::prelude::*;
use rand::prelude::*;

#[derive(Resource, Debug)]
pub struct Seed {
    pub rng: StdRng,
    pub key: u64,
}

impl Default for Seed {
    fn default() -> Self {
        let key: u64 = rand::random();
        let rng = StdRng::seed_from_u64(key);

        Seed { rng, key }
    }
}

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

    /// Returns previous intensity level.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let mut weather = Weather::Drizzle;
    ///
    /// weather = weather.lessen();
    ///
    /// assert_eq!(result, Weather::Clear);
    /// ```
    pub fn lessen(&self) -> Self {
        match self {
            Weather::HeavyRain => Weather::Rain,
            Weather::Rain => Weather::Drizzle,
            Weather::Drizzle => Weather::Clear,
            Weather::Clear => Weather::Clear,
        }
    }

    /// Returns next intensity level.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let mut weather = Weather::Clear;
    ///
    /// weather = weather.intensify();
    ///
    /// assert_eq!(result, Weather::Drizzle);
    /// ```
    pub fn intensify(&self) -> Self {
        match self {
            Weather::Clear => Weather::Drizzle,
            Weather::Drizzle => Weather::Rain,
            Weather::Rain => Weather::HeavyRain,
            Weather::HeavyRain => Weather::HeavyRain,
        }
    }
}
