extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::types::Width;
// need to use 'use' not just extern crate
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

pub struct WindowsSize   {
    height: u32,
    width: u32,
}

const WINDOWS_SIZE : WindowsSize = WindowsSize {
    height: 200,
    width: 200,
};

pub struct WindowsGridCellSize  {
    height: i32,
    width: i32,
}

const WINDOWSGRIDCELLSIZE : WindowsGridCellSize = WindowsGridCellSize   {
    height: 20,
    width: 20,
};

#[derive(Clone, PartialEq)]
enum Direction  {
    Right, Left, Up, Down
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    snake: Snake,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, args);
    }

    fn update(&mut self)
    {
        self.snake.update();
    }

	fn pressed(&mut self, btn: &Button)  {
		let last_direction = self.snake.direction.clone();

		self.snake.direction = match btn  {
			&Button::Keyboard(Key::Up)
				if last_direction != Direction::Down => Direction::Up,
			&Button::Keyboard(Key::Down)
				if last_direction != Direction::Up => Direction::Down,
			&Button::Keyboard(Key::Right)
				if last_direction != Direction::Left => Direction::Right,
			&Button::Keyboard(Key::Left)
				if last_direction != Direction::Right => Direction::Left,
			_ => last_direction
		};
	}

}

struct Snake    {
	body: LinkedList<(i32, i32)>,
	direction: Direction,
}

impl Snake  {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

		let squares: Vec<graphics::types::Rectangle> = self.body
			.iter()
			.map(|&(x, y)|  {
				graphics::rectangle::square(
					(x * WINDOWSGRIDCELLSIZE.height) as f64,
					(y * WINDOWSGRIDCELLSIZE.width) as f64,
					20_f64);
			})
			.collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
			squares.into_iter()
				.for_each(|square| graphics::rectangle(RED, square, transform, gl));

        });
    }

    fn update(&mut self)  {
		let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.direction    {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }
    }
}

fn main() {

    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [WINDOWS_SIZE.height, WINDOWS_SIZE.width]
    ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App{
        gl: GlGraphics::new(opengl),
        snake: Snake  {
			body: LinkedList::from_iter(vec![(0,0), (0,1)].into_iter()), 
			direction: Direction::Right},
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(u) = e.update_args()	{
            app.update();
        }

		if let Some(k) = e.button_args()	{
			if k.state == ButtonState::Press  {
				app.pressed(&k.button);
			}
		}
    }
}

