use aitios_geom::Vector3;
use aitios_scene::Entity;
use std::rc::Rc;

pub fn translate(entity: &mut Entity, offset: Vector3<f32>) {
    // Clone mesh, if shared with other entities, otherwise edit in place.
    offset_position_vector(&mut Rc::make_mut(&mut entity.mesh).positions, offset);
}

fn offset_position_vector(positions: &mut [f32], offset: Vector3<f32>) {
    positions.chunks_mut(3).for_each(|pos| {
        pos[0] += offset.x;
        pos[1] += offset.y;
        pos[2] += offset.z;
    });
}
