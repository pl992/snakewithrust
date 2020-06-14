use ggez::{Context, ContextBuilder, GameResult};
use ggez::input::keyboard::{KeyCode,KeyMods};
use ggez::event::{self, EventHandler};
use ggez::graphics;

mod snake;
mod food;
mod collisions;
use crate::collisions::CheckCollisions;

struct State {
    snake: snake::Snake,
    food: food::Food,
    score: u32,
}

fn main() {
    let (mut ctx, mut event_loop) =
       ContextBuilder::new("game_name", "author_name")
           .build()
           .unwrap();
    let player = snake::Snake::new([10.,30.]);
    let mut state = State{snake:player,food:food::Food::new(graphics::Rect::new(50.,50.,10.,10.))
        ,score:0,};

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        let screen = graphics::screen_coordinates(_ctx);
        let screen_size = [screen.w,screen.h];
        let eaten = self.food.check_collision(&self.snake.get_head(),screen_size);
        if eaten {
            self.score += 1;
        }
        self.snake.move_snake(eaten,_ctx);
        if self.snake.check_collision(&self.snake.get_head(),screen_size) {
            event::quit(_ctx);
        }
        Ok(())
    }
    fn key_down_event(&mut self,_ctx:&mut Context,keycode:KeyCode,_keymods:KeyMods,_repeat:bool) {
        self.snake.change_direction(keycode);

    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let score = &graphics::Text::new(String::from(self.score.to_string()));
        graphics::draw(ctx,score,graphics::DrawParam::default())?;
        self.snake.draw(ctx)?;
        self.food.draw(ctx)?;
        graphics::present(ctx)
    }
}
