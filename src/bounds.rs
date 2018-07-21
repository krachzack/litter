use aitios_geom::Aabb;
use aitios_scene::{Entity, Mesh};

pub fn scene_bounds(scene: &Vec<Entity>) -> Aabb {
    let vertices = scene
        .iter()
        .flat_map(|e| e.mesh.vertices())
        .map(|v| v.position);

    Aabb::from_points(vertices)
}
