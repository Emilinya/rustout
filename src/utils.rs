use egui::{Rect, Vec2};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum BounceDirection {
    Up,
    Left,
    Down,
    Right,
}

/// Check collision between some moving object with a bounding box and the
/// edges of some boundary. If the boundary rectangle does not fully contain
/// the bounding box, return the direction the object must bounce to stay within
/// the boundary.
pub fn collide_with_boundary(bounding_box: &Rect, boundary: &Rect) -> Option<BounceDirection> {
    if !boundary.contains_rect(*bounding_box) {
        if bounding_box.left() < boundary.left() {
            Some(BounceDirection::Right)
        } else if bounding_box.right() > boundary.right() {
            Some(BounceDirection::Left)
        } else if bounding_box.top() < boundary.top() {
            Some(BounceDirection::Down)
        } else if bounding_box.bottom() > boundary.bottom() {
            Some(BounceDirection::Up)
        } else {
            eprintln!("WARNING: outside but not colliding?");
            None
        }
    } else {
        None
    }
}

/// Check collision between an object moving in `self_direction` with a bounding box
/// `self_bounding_box`, and some other bounding box. If bounding boxes intersect,
/// return the direction the moving object should bounce, otherwise, return None.
pub fn collide_with_rect(
    self_direction: &Vec2,
    self_bounding_box: &Rect,
    other_bounding_box: &Rect,
) -> Option<BounceDirection> {
    if self_bounding_box.intersects(*other_bounding_box) {
        let inside_bottom = -(self_bounding_box.top() - other_bounding_box.bottom());
        let inside_right = -(self_bounding_box.left() - other_bounding_box.right());
        let inside_left = self_bounding_box.right() - other_bounding_box.left();
        let inside_top = self_bounding_box.bottom() - other_bounding_box.top();

        let mut min_in: f32 = 999.0;
        if inside_bottom >= 0.0 && self_direction.y < 0.0 {
            min_in = min_in.min(inside_bottom);
        }
        if inside_right >= 0.0 && self_direction.x < 0.0 {
            min_in = min_in.min(inside_right);
        }
        if inside_left >= 0.0 && self_direction.x > 0.0 {
            min_in = min_in.min(inside_left);
        }
        if inside_top >= 0.0 && self_direction.y > 0.0 {
            min_in = min_in.min(inside_top);
        }

        if min_in == inside_bottom {
            Some(BounceDirection::Down)
        } else if min_in == inside_right {
            Some(BounceDirection::Right)
        } else if min_in == inside_left {
            Some(BounceDirection::Left)
        } else if min_in == inside_top {
            Some(BounceDirection::Up)
        } else {
            eprintln!("WARNING: colliding but not inside?");
            None
        }
    } else {
        None
    }
}
