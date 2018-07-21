use aitios_geom::Vector3;
use aitios_scene::Entity;
use bounds::scene_bounds;
use filter::Filter;
use std::rc::Rc;
use transform::translate;

pub struct Grid {
    /// Duplications in x, y, z direction, respectively.
    /// x times y times z duplications total.
    dimensions: Vector3<usize>,
    cell_size_x: Option<f32>,
    cell_size_y: Option<f32>,
    cell_size_z: Option<f32>,
}

impl Grid {
    pub fn new(
        x_clones: usize,
        y_clones: usize,
        z_clones: usize,
        cell_size_x: Option<f32>,
        cell_size_y: Option<f32>,
        cell_size_z: Option<f32>,
    ) -> Self {
        Grid {
            dimensions: Vector3::new(x_clones, y_clones, z_clones),
            cell_size_x,
            cell_size_y,
            cell_size_z,
        }
    }
}

impl Filter for Grid {
    fn apply(&self, scene: &mut Vec<Entity>) {
        let bounds = scene_bounds(scene);
        let scene_size = bounds.max - bounds.min;
        let cell_size = Vector3::new(
            self.cell_size_x.unwrap_or(scene_size.x),
            self.cell_size_y.unwrap_or(scene_size.y),
            self.cell_size_z.unwrap_or(scene_size.z),
        );

        let mut new_scene = Vec::with_capacity(
            scene.len() * self.dimensions.x * self.dimensions.y * self.dimensions.z,
        );

        let min_x_offset = -0.5 * ((self.dimensions.x - 1) as f32);
        let min_y_offset = -0.5 * ((self.dimensions.y - 1) as f32);
        let min_z_offset = -0.5 * ((self.dimensions.z - 1) as f32);

        for x in 0..self.dimensions.x {
            for y in 0..self.dimensions.y {
                for z in 0..self.dimensions.z {
                    let offset = Vector3::new(
                        cell_size.x * ((x as f32) + min_x_offset),
                        cell_size.y * ((y as f32) + min_y_offset),
                        cell_size.z * ((z as f32) + min_z_offset),
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
                mesh: Rc::clone(&e.mesh),
            })
            .map(|mut e| {
                translate(&mut e, offset);
                e
            }),
    );
}
