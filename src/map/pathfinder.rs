use super::prelude::*;
use crate::npc::prelude::*;
use crate::prelude::*;

#[derive(Default)]
pub struct Pathfinder {
    map: Map,
    stats: NpcStats,
    rng: Option<StdRng>,
    weather: Weather,
}

impl Pathfinder {
    pub fn new() -> Self {
        Pathfinder::default()
    }

    pub fn with_map(&mut self, map: Map) -> &mut Self {
        self.map = map;
        self
    }

    pub fn with_stats(&mut self, stats: NpcStats) -> &mut Self {
        self.stats = stats;
        self
    }

    pub fn with_rng(&mut self, rng: StdRng) -> &mut Self {
        self.rng = Some(rng);
        self
    }

    pub fn with_weather(&mut self, weather: Weather) -> &mut Self {
        self.weather = weather;
        self
    }

    pub fn find_path(&self, start: MapNode, goal: MapNode) -> Option<(Vec<MapNode>, u32)> {
        astar(
            &start,
            |p| {
                self.get_successors(p)
                    .iter()
                    .map(|s| (s.node, s.cost))
                    .collect::<Vec<_>>()
            },
            |p| ((p.0 - goal.0).abs() + (p.1 - goal.1).abs()) as u32,
            |p| *p == goal,
        )
    }

    pub fn find_path_by_vec2(&self, start: Vec2, goal: Vec2) -> Option<(Vec<Vec2>, u32)> {
        let start = self.map.vec2_to_node(&start);
        let node_goal = self.map.vec2_to_node(&goal);

        if let Some((nodes, cost)) = self.find_path(start, node_goal) {
            let mut instructions: Vec<Vec2> = nodes
                .iter()
                .map(|node| self.map.node_to_vec2(*node))
                .collect();

            if let Some(rng) = &self.rng {
                let variation = TILE_SIZE as f32 / 2.;
                let variation_range = -variation..variation;
                let mut last_step = instructions[0];
                let mut rng = rng.to_owned();

                let mut iter = instructions.iter_mut().peekable();
                let mut instructions = Vec::new();

                while let Some(mut step) = iter.next() {
                    if iter.peek().is_some() {
                        let x_range = (last_step.x - variation)..(last_step.x + variation);
                        let y_range = (last_step.y - variation)..(last_step.y + variation);

                        if !x_range.contains(&step.x) {
                            step.x += rng.gen_range(variation_range.clone());
                        }

                        if !y_range.contains(&step.y) {
                            step.y += rng.gen_range(variation_range.clone());
                        }
                    } else {
                        *step = goal;
                    }

                    last_step = *step;
                    instructions.push(*step);
                }
            }

            return Some((instructions, cost));
        }

        None
    }

    pub fn is_successor_ignored(&self, x: i16, y: i16) -> bool {
        // Ignores middle and diagonals
        x + y == 0 || (x + y).abs() == 2
    }

    pub fn get_successors(&self, node: &MapNode) -> Vec<Successor> {
        let mut successors = Vec::new();
        for dx in -1i16..=1 {
            for dy in -1i16..=1 {
                if self.is_successor_ignored(dx, dy) {
                    continue;
                }

                let next_node = MapNode(node.0 + dx, node.1 + dy);
                if next_node.0 < 0
                    || next_node.0 >= self.map.width.into()
                    || next_node.1 < 0
                    || next_node.1 >= self.map.height.into()
                {
                    continue;
                }

                let map_value = self.map.data[next_node.1 as usize][next_node.0 as usize];
                if let Some(node) = map_value {
                    let fear = i8::max(0, node.fear_cost as i8 - self.stats.guts as i8) as u8;
                    let mut cost = (node.path_cost + fear) as u32;

                    if !self.weather.is_clear() {
                        let coverage = match node.roof {
                            0 => 6,
                            _ => node.roof,
                        } as i8;

                        let extra_weather_cost =
                            coverage * self.weather as i8 - self.stats.guts as i8;

                        if extra_weather_cost > 0 {
                            cost += extra_weather_cost as u32;
                        }
                    }

                    successors.push(Successor {
                        node: next_node,
                        cost,
                    });
                }
            }
        }

        successors
    }
}
