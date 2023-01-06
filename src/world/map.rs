use super::prelude::*;
use pathfinding::prelude::astar;
use rand::prelude::*;

const TILE_SIZE: u8 = 16;

#[derive(Debug, PartialEq, Clone)]
pub struct EntityData {
    pub identifier: String,
    pub position: Vec2,
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd)]
pub struct MapNode(pub i16, pub i16);

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub node: MapNode,
    pub cost: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct NodeData {
    pub path_cost: u8,
    pub fear_cost: u8,
    pub roof: u8,
}

#[derive(Resource, Default, Clone)]
pub struct Map {
    pub width: u8,
    pub height: u8,
    pub data: Vec<Vec<Option<NodeData>>>,
    pub entities: Vec<EntityData>,
}

impl Map {
    pub fn from_ldtk(file: &str) -> Self {
        let ldtk = ldtk_rust::Project::new(file);
        let mut data: Vec<Vec<Option<NodeData>>> = Vec::new();
        let mut entities: Vec<EntityData> = Vec::new();

        if let Some(level) = ldtk.get_level(0) {
            if let Some(layers) = &level.layer_instances {
                let path_cost_layer = layers.iter().find(|l| l.identifier == "PathCost").unwrap();
                let fear_cost_layer = layers.iter().find(|l| l.identifier == "FearCost").unwrap();
                let roof_layer = layers.iter().find(|l| l.identifier == "Roof").unwrap();
                let walls_layer = layers.iter().find(|l| l.identifier == "Walls").unwrap();
                let entities_layer = layers.iter().find(|l| l.identifier == "Entities").unwrap();

                let mut row: Vec<Option<NodeData>> = Vec::new();

                for (idx, path_cost) in path_cost_layer.int_grid_csv.iter().enumerate() {
                    let next_item = match walls_layer.int_grid_csv.get(idx) {
                        Some(0) => Some(NodeData {
                            path_cost: *path_cost as u8,
                            fear_cost: *fear_cost_layer.int_grid_csv.get(idx).unwrap() as u8,
                            roof: *roof_layer.int_grid_csv.get(idx).unwrap() as u8,
                        }),
                        _ => None,
                    };

                    row.push(next_item);

                    if (idx + 1) % (path_cost_layer.c_wid as usize) == 0 {
                        data.push(row.clone());
                        row.clear();
                    }
                }

                let map_height = (entities_layer.c_hei * entities_layer.grid_size) as f32;

                for entity in entities_layer.entity_instances.iter() {
                    let identifier = entity.identifier.clone();
                    let position = Vec2::new(entity.px[0] as f32, map_height - entity.px[1] as f32);

                    entities.push(EntityData {
                        identifier,
                        position,
                        width: entity.width,
                        height: entity.height,
                    })
                }
            }
        }

        data.reverse();

        Self {
            width: data[0].len() as u8,
            height: data.len() as u8,
            entities,
            data,
        }
    }

    pub fn vec2_to_node(&self, translation: &Vec2) -> MapNode {
        MapNode(
            translation.x.round() as i16 / TILE_SIZE as i16,
            translation.y.round() as i16 / TILE_SIZE as i16,
        )
    }

    pub fn node_to_vec2(&self, node: MapNode) -> Vec2 {
        let MapNode(x, y) = node;

        Vec2::new(x as f32 * TILE_SIZE as f32, y as f32 * TILE_SIZE as f32)
    }
}

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
        let goal = self.map.vec2_to_node(&goal);

        if let Some((nodes, cost)) = self.find_path(start, goal) {
            let mut instructions: Vec<Vec2> = nodes
                .iter()
                .map(|node| self.map.node_to_vec2(*node))
                .collect();

            if let Some(rng) = &self.rng {
                let variation = TILE_SIZE as f32 / 2.;
                let variation_range = -variation..variation;
                let mut last_step = instructions[0];
                let mut rng = rng.to_owned();

                instructions = instructions
                    .iter_mut()
                    .map(|mut step| {
                        let x_range = (last_step.x - variation)..(last_step.x + variation);
                        let y_range = (last_step.y - variation)..(last_step.y + variation);

                        if !x_range.contains(&step.x) {
                            step.x += rng.gen_range(variation_range.clone());
                        }

                        if !y_range.contains(&step.y) {
                            step.y += rng.gen_range(variation_range.clone());
                        }

                        last_step = *step;

                        *step
                    })
                    .collect();
            }

            return Some((instructions, cost));
        }

        None
    }

    pub fn get_successors(&self, node: &MapNode) -> Vec<Successor> {
        let mut successors = Vec::new();
        for dx in -1i16..=1 {
            for dy in -1i16..=1 {
                if dx == 0 && dy == 0 {
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
