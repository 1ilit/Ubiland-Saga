use crate::texture::{AnimatedTexture, Texture, Rect};

pub fn overlap_x(a: Rect, b: Rect) -> bool {
    if a.x + a.w / 2.0 <= b.x - b.w / 2.0 {
        return false;
    }

    if a.x - a.w / 2.0 >= b.x + b.w / 2.0 {
        return false;
    }

    true
}

pub fn intersect(a: &Texture, b: &AnimatedTexture) -> bool {
    if a.y - a.height / 2.0 >= b.y + b.height / 2.0 {
        return false;
    }

    if a.y + a.height / 2.0 <= b.y - b.height / 2.0 {
        return false;
    }

    if a.x + a.width / 2.0 <= b.x - b.width / 2.0 {
        return false;
    }

    if a.x - a.width / 2.0 >= b.x + b.width / 2.0 {
        return false;
    }

    true
}
