use aitios_geom::Vector3;
use aitios_scene::{DeinterleavedIndexedMeshBuf, Entity};
use std::rc::Rc;

pub fn translate(entity: &mut Entity, offset: Vector3<f32>) {
    entity.mesh = Rc::new(DeinterleavedIndexedMeshBuf {
        positions: offset_position_vector(entity.mesh.positions.clone(), offset),
        normals: entity.mesh.normals.clone(),
        texcoords: entity.mesh.texcoords.clone(),
        indices: entity.mesh.indices.clone(),
    });
}

fn offset_position_vector(mut positions: Vec<f32>, offset: Vector3<f32>) -> Vec<f32> {
    positions.chunks_mut(3).for_each(|pos| {
        pos[0] += offset.x;
        pos[1] += offset.y;
        pos[2] += offset.z;
    });
    positions
}
