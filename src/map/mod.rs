pub mod pathfinder;
pub mod prelude;

pub use crate::prelude::*;
pub use prelude::*;

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
                    let fear_cost = match fear_cost_layer.int_grid_csv.get(idx) {
                        Some(0) => 1,
                        Some(v) => *v,
                        None => 1,
                    } as u8;

                    let next_item = match walls_layer.int_grid_csv.get(idx) {
                        Some(0) => Some(NodeData {
                            path_cost: *path_cost as u8,
                            fear_cost,
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
