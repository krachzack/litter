use aitios_geom::{Aabb, Position, Vector3};
use aitios_scene::{DeinterleavedIndexedMeshBuf, Entity, Mesh};
use filter::Filter;
use std::rc::Rc;

pub struct Grid {
    /// Duplications in x, y, z direction, respectively.
    /// x times y times z duplications total.
    dimensions: Vector3<usize>,
}

impl Grid {
    pub fn new(x_clones: usize, y_clones: usize, z_clones: usize) -> Self {
        Grid {
            dimensions: Vector3::new(x_clones, y_clones, z_clones),
        }
    }
}

impl Filter for Grid {
    fn apply(&self, scene: &mut Vec<Entity>) {
        let bounds = scene_bounds(scene);
        let scene_size = bounds.max - bounds.min;
        let scene_center = bounds.min + 0.5 * scene_size;

        let mut new_scene = Vec::with_capacity(
            scene.len() * self.dimensions.x * self.dimensions.y * self.dimensions.z,
        );

        for x in 0..self.dimensions.x {
            for y in 0..self.dimensions.y {
                for z in 0..self.dimensions.z {
                    let offset = -scene_center
                        + Vector3::new(
                            scene_size.x * ((x as f32) - 0.5 * ((self.dimensions.x - 1) as f32)),
                            scene_size.y * ((y as f32) - 0.5 * ((self.dimensions.y - 1) as f32)),
                            scene_size.z * ((z as f32) - 0.5 * ((self.dimensions.z - 1) as f32)),
                        );

                    add_clone_offset(
                        &mut new_scene,
                        &scene,
                        offset,
                        &format!("grid-{}-{}-{}", x, y, z),
                    )
                }
            }
        }

        *scene = new_scene;
    }
}

fn add_clone_offset(
    target_scene: &mut Vec<Entity>,
    additional_entities: &Vec<Entity>,
    offset: Vector3<f32>,
    new_name_postfix: &str,
) {
    target_scene.extend(
        additional_entities
            .iter()
            .enumerate()
            .map(|(idx, e)| Entity {
                name: format!(
                    "{orig}-{seq}-{post}",
                    orig = e.name,
                    seq = idx,
                    post = new_name_postfix
                ),
                material: Rc::clone(&e.material),
                mesh: Rc::new(DeinterleavedIndexedMeshBuf {
                    positions: offset_position_vector(e.mesh.positions.clone(), offset),
                    normals: e.mesh.normals.clone(),
                    texcoords: e.mesh.texcoords.clone(),
                    indices: e.mesh.indices.clone(),
                }),
            }),
    );
}

fn offset_position_vector(mut positions: Vec<f32>, offset: Vector3<f32>) -> Vec<f32> {
    positions.chunks_mut(3).for_each(|pos| {
        pos[0] += offset.x;
        pos[1] += offset.y;
        pos[2] += offset.z;
    });

    positions
}

fn scene_bounds(scene: &Vec<Entity>) -> Aabb {
    let vertices = scene
        .iter()
        .flat_map(|e| e.mesh.vertices())
        .map(|v| v.position());

    Aabb::from_points(vertices)
}
