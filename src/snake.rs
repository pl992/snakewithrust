use ggez::{Context, GameResult};
use ggez::input::keyboard::{KeyCode};
use ggez::graphics;
use crate::collisions;
use crate::collisions::CheckCollisions;

const SIZE:f32 = 10.;

#[derive(PartialEq,Copy,Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
pub struct Snake {
    head:[f32;2],
    body:Vec<graphics::Rect>,
    direction:Direction,
}

impl CheckCollisions for Snake {
    fn check_collision(&mut self,comparison:&graphics::Rect,screen_size:[f32;2]) -> bool{
        let mut eaten = false;
        for i in &self.body[1..] {
            eaten = collisions::get_collision(i,comparison);
            if eaten == true{
                break;
            }
        }
        eaten
    }
}
impl Snake {
    pub fn new(head: [f32; 2]) -> Self {
        let mut rect: Vec<graphics::Rect> = Vec::new();
        rect.push(graphics::Rect{x:head[0], y:head[1], w:SIZE,h:SIZE});
        Self {
            head,
            body: rect,
            direction: Direction::RIGHT,
        }
    }

    pub fn get_head(&self) -> graphics::Rect {
        self.body[0]
    }

    pub fn move_snake( &mut self ,eaten:bool,_ctx:& Context) {
        let screen = &graphics::screen_coordinates(_ctx);
        let screen_size = [screen.w,screen.h];
        match self.direction {
            Direction::DOWN => self.head[1] += SIZE,
            Direction::UP => self.head[1] -= SIZE,
            Direction::LEFT => self.head[0] -= SIZE,
            Direction::RIGHT => self.head[0] += SIZE,
        }
        for i in 0..2 {
            if self.head[i] > screen_size[i] {
                self.head[i] = self.head[i] - screen_size[i];
            } else if self.head[i] < 0. {
                self.head[i] = screen_size[i] - self.head[i];
            }
        }
        self.body.insert(0,graphics::Rect{x:self.head[0],y:self.head[1],w:SIZE,h:SIZE});
        if ! eaten {
            self.body.pop();
        }
    }

    fn invert_direction(&self) -> Direction{
        match self.direction {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }
    pub fn change_direction(&mut self,keycode:KeyCode) {
        let d = match keycode {
            KeyCode::Up => Direction::UP,
            KeyCode::Down => Direction::DOWN,
            KeyCode::Left => Direction::LEFT,
            KeyCode::Right => Direction::RIGHT,
            _ => self.direction,
        };
        let inverted_d = self.invert_direction();
        if d != inverted_d {
            self.direction = d;
        }
    }

    pub fn draw(&mut self,ctx: &mut Context) -> GameResult {
        for i in &self.body {
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                *i,
                graphics::WHITE
            )?;
            graphics::draw(ctx,&rect,graphics::DrawParam::default())?;
        }
        Ok(())
    }
}
