use aitios_scene::Entity;
use bounds::scene_bounds;
use filter::Filter;
use transform::translate;

#[derive(Copy, Clone)]
pub enum Anchor {
    Above,
    Under,
    Center,
}

pub struct Align {
    /// Alignment on Y/Z plane, `Above` meaning, all geometry lands
    /// in positive X, and `Under` meaning everything in negative X.
    /// `Center` places everything equally around X and might be what
    /// you want.
    axis_x: Anchor,
    /// Alignment on X/Z plane, `Above` meaning, all geometry lands
    /// in positive Y, and `Under` meaning everything in negative Y.
    /// `Center` places everything equally around Y.
    /// You may want `Above` here, so every vertex is in the top
    /// half of the world.
    axis_y: Anchor,
    /// Alignment on X/Y plane, `Above` meaning, all geometry lands
    /// in positive Z, and `Under` meaning everything in negative Z.
    /// `Center` places everything equally around Z and might be what
    /// you want.
    axis_z: Anchor,
}

impl Align {
    pub fn new(axis_x: Anchor, axis_y: Anchor, axis_z: Anchor) -> Self {
        Align {
            axis_x,
            axis_y,
            axis_z,
        }
    }
}

impl Filter for Align {
    fn apply(&self, scene: &mut Vec<Entity>) {
        let bounds = scene_bounds(scene);
        let scene_size = bounds.max - bounds.min;
        let scene_center = bounds.min + 0.5 * scene_size;

        let offset = {
            let mut o = -scene_center;
            o.x += scene_size.x * anchor_offset_factor(self.axis_x);
            o.y += scene_size.y * anchor_offset_factor(self.axis_y);
            o.z += scene_size.z * anchor_offset_factor(self.axis_z);
            o
        };

        scene.iter_mut().for_each(|e| translate(e, offset));
    }
}

fn anchor_offset_factor(anchor: Anchor) -> f32 {
    match anchor {
        Anchor::Center => 0.0,
        Anchor::Above => 0.5,
        Anchor::Under => -0.5,
    }
}
