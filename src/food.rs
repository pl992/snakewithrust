use ggez::{Context, GameResult};
use ggez::graphics;
use rand;
use rand::Rng;
use crate::collisions;
use crate::collisions::CheckCollisions;

pub struct Food {
    rect: graphics::Rect,
}

impl CheckCollisions for Food {
    fn check_collision(&mut self,comparison:&graphics::Rect,screen_size:[f32;2]) -> bool {
        let eaten = collisions::get_collision(&self.rect,comparison);
        let mut rng = rand::thread_rng();
        if eaten {
            let x: f32 = rng.gen();
            let y: f32 = rng.gen();
            self.rect.x = x * screen_size[0];
            self.rect.y = y * screen_size[1];
        }
        eaten

    }
}
impl Food {
    pub fn new(rect: graphics::Rect) -> Self {
        Self {
            rect,
        }
    }
    pub fn draw(&self,ctx:&mut Context) -> GameResult {
        let mesh = &graphics::Mesh::new_rectangle(ctx,graphics::DrawMode::fill(),self.rect,
                                                 graphics::Color{r:1.,g:0.,b:0.,a:1.,})?;
        graphics::draw(ctx,mesh,graphics::DrawParam::default())
    }
}