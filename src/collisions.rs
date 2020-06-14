use ggez::graphics;

pub fn get_collision(reference: &graphics::Rect, comparison: &graphics::Rect) -> bool {
    let ref_left = reference.x;
    let ref_right = reference.x + reference.w;
    let ref_down = reference.y;
    let ref_up = reference.y + reference.h;
    let comp_left = comparison.x - reference.w;
    let comp_right = comparison.x + comparison.w + reference.w;
    let comp_down = comparison.y - reference.h;
    let comp_up = comparison.y + comparison.h + reference.h;
    let mut eaten = false;
    if ref_left > comp_left && ref_right < comp_right
        && ref_up < comp_up && ref_down > comp_down {
        eaten = true;
    }
    eaten
}

pub trait CheckCollisions {
    fn check_collision(&mut self, comparison: &graphics::Rect, screen_size:[f32;2]) -> bool;
}
