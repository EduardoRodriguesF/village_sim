use bevy::prelude::*;
use pathfinding::prelude::astar;
use rand::prelude::*;

const TILE_SIZE: u8 = 16;

pub struct EntityData {
    pub identifier: String,
    pub position: Vec2,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd)]
pub struct MapNode(pub i16, pub i16);

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub node: MapNode,
    pub cost: u32,
}

#[derive(Resource)]
pub struct Map {
    pub width: u8,
    pub height: u8,
    pub data: Vec<Vec<Option<u8>>>,
    pub entities: Vec<EntityData>,
}

impl Map {
    pub fn new(map_lines: Vec<String>, entities: Vec<EntityData>) -> Self {
        let width = map_lines[0].len() as u8;
        let height = map_lines.len() as u8;
        let mut data = Vec::new();

        for line in map_lines {
            let mut row: Vec<Option<u8>> = Vec::new();

            for c in line.chars() {
                match c {
                    '1'..='9' => row.push(Some(c as u8 - b'0')),
                    _ => row.push(None),
                }
            }

            data.push(row);
        }

        Self {
            width,
            height,
            data,
            entities,
        }
    }

    pub fn from_ldtk(file: &str) -> Self {
        let ldtk = ldtk_rust::Project::new(file);
        let mut data: Vec<String> = Vec::new();
        let mut entities: Vec<EntityData> = Vec::new();

        if let Some(level) = ldtk.get_level(0) {
            if let Some(layers) = &level.layer_instances {
                let path_cost = layers.iter().find(|l| l.identifier == "PathCost").unwrap();
                let walls = layers.iter().find(|l| l.identifier == "Walls").unwrap();
                let entities_layer = layers.iter().find(|l| l.identifier == "Entities").unwrap();

                let mut row = String::new();

                for (idx, grid_item) in path_cost.int_grid_csv.iter().enumerate() {
                    let next_item = match walls.int_grid_csv.get(idx) {
                        Some(0) => grid_item.to_string(),
                        _ => String::from("x"),
                    };

                    row.push_str(&next_item);

                    if (idx + 1) % (path_cost.c_wid as usize) == 0 {
                        data.push(row.clone());
                        row.clear();
                    }
                }

                let grid_size = Vec2::splat(entities_layer.grid_size as f32);

                for entity in entities_layer.entity_instances.iter() {
                    let identifier = entity.identifier.clone();
                    let position =
                        Vec2::new(entity.grid[0] as f32, entity.grid[1] as f32) * grid_size;

                    entities.push(EntityData {
                        identifier,
                        position,
                    })
                }
            }
        }

        Self::new(data, entities)
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
                    || next_node.0 >= self.width.into()
                    || next_node.1 < 0
                    || next_node.1 >= self.height.into()
                {
                    continue;
                }

                let map_value = self.data[next_node.1 as usize][next_node.0 as usize];
                if let Some(value) = map_value {
                    successors.push(Successor {
                        node: next_node,
                        cost: value as u32,
                    });
                }
            }
        }

        successors
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

    pub fn find_path_by_vec2(
        &self,
        start: Vec2,
        goal: Vec2,
        rng: Option<&mut StdRng>,
    ) -> Option<(Vec<Vec2>, u32)> {
        let start = Self::vec2_to_node(&start);
        let goal = Self::vec2_to_node(&goal);

        if let Some((nodes, cost)) = self.find_path(start, goal) {
            let mut instructions: Vec<Vec2> =
                nodes.iter().map(|node| Self::node_to_vec2(*node)).collect();

            if let Some(rng) = rng {
                let variation = TILE_SIZE as f32 / 2.;
                let variation_range = -variation..variation;
                let mut last_step = instructions[0];

                instructions = instructions.iter_mut().map(|mut step| {
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
                }).collect();
            }

            return Some((instructions, cost));
        }

        None
    }

    pub fn vec2_to_node(translation: &Vec2) -> MapNode {
        MapNode(
            translation.x.round() as i16 / TILE_SIZE as i16,
            translation.y.round() as i16 / TILE_SIZE as i16,
        )
    }

    pub fn node_to_vec2(node: MapNode) -> Vec2 {
        let MapNode(x, y) = node;

        Vec2::new(x as f32 * TILE_SIZE as f32, y as f32 * TILE_SIZE as f32)
    }
}
