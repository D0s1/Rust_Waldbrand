use ggez;
use ggez::conf;
use ggez::event;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, MeshBuilder, Rect};
use ggez::input::keyboard;
use ggez::nalgebra;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use rand::{Rand, Rng};
use std::{thread, cmp, time::Duration};

const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 1000.0;
const ENTRY_WIDTH: f32 = SCREEN_WIDTH / 100.0;
const ENTRY_HEIGHT: f32 = SCREEN_HEIGHT / 100.0;
const FIRE_AGE: u32 = 300;
const BURNT_AGE: u32 = 650;
const FIRE_PROB: f32 = 0.0025;
const TREE_PROB: f32 = 0.000005;
const EMPTY_PROB: f32 = 0.0002;

#[derive(Debug, Copy, Clone)]
enum Entry {
    Edge,
    Empty,
    Tree,
    Water,
    Fire(u32),
    Burned(u32),
}

impl Rand for Entry {
    fn rand<R: Rng>(rng: &mut R) -> Self {
	match rng.gen::<f32>() {
	    i if i < 0.1 => Entry::Empty,
	    _ => Entry::Tree,
	}
    }
}

struct State {
    grid: [[Entry; 100]; 100],
    pause: bool,
    sped_up: bool,
    neighbours: [(i8, i8); 8],
    fire_prob: f32,
    tree_prob: f32,
    empty_prob: f32,
    fire_fw: u32,
    burned_fw: u32,
}

impl State {
    fn new(_ctx: &mut Context) -> State {
	let mut grid = [[Entry::Edge; 100]; 100];
	State::set_grid(&mut grid);
	State {
	    grid,
	    pause: false,
	    sped_up: false,
	    neighbours: [
		(-1, -1),
		(-1, 0),
		(-1, -1),
		(0, -1),
		(0, 1),
		(1, -1),
		(1, 0),
		(1, 1),
	    ],
	    fire_prob: FIRE_PROB,
	    tree_prob: TREE_PROB,
	    empty_prob: EMPTY_PROB,
	    fire_fw: 0,
	    burned_fw: 0,
	}
    }

    fn set_grid(grid: &mut [[Entry; 100]; 100]) {
	let mut rng = rand::thread_rng();
	for y in 1..99 {
	    for x in 1..99 {
		grid[y][x] = rng.gen();
	    }
	}
	let rand_x = rng.gen_range(1, 99);
	let rand_y = rng.gen_range(1, 99);
	grid[rand_x][rand_y] = Entry::Fire(FIRE_AGE);
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

	if !timer::check_update_time(ctx, 60) { return Ok(()); }

	if self.sped_up {
	    self.sped_up = false;
	    self.fire_prob = FIRE_PROB;
	    self.tree_prob = TREE_PROB;
	    self.empty_prob = EMPTY_PROB;
	    self.fire_fw = 0;
	    self.burned_fw = 0;
	}

	if keyboard::is_key_pressed(ctx, KeyCode::P) {
	    self.pause = !self.pause;
	    thread::sleep(Duration::from_millis(500));
	}

	if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
	    self.sped_up = true;
	    self.fire_prob = FIRE_PROB * 20.0;
	    self.tree_prob = TREE_PROB * 100.0;
	    self.empty_prob = EMPTY_PROB * 5.0;

	    self.burned_fw = 550;
	    self.fire_fw = 20
	}

	if keyboard::is_mod_active(ctx, KeyMods::ALT) {
	    let mouse_position = ggez::input::mouse::position(ctx);
	    let x = (mouse_position.x / 10.0) as usize;
	    let y = (mouse_position.y / 10.0) as usize;
	    if !(x >= 99 || y >= 99 || x == 0 || y == 0) {
		self.grid[x][y] = Entry::Fire(FIRE_AGE);
	    }
	}

	if keyboard::is_mod_active(ctx, KeyMods::CTRL) {
	    let mouse_position = ggez::input::mouse::position(ctx);
	    let x = (mouse_position.x / 10.0) as usize;
	    let y = (mouse_position.y / 10.0) as usize;
	    if !(x >= 99 || y >= 99 || x == 0 || y == 0) {
		self.grid[x][y] = Entry::Water;
	    }
	}

	if keyboard::is_key_pressed(ctx, KeyCode::R) {
	    State::set_grid(&mut self.grid);
	}

	if self.pause { return Ok(()); }
	for y in 1..99 {
	    for x in 1..99 {
		self.grid[x][y] = match self.grid[x][y] {
		    Entry::Tree => {
			let mut ret = Entry::Tree;
			for (dx, dy) in self.neighbours.iter() {
			    if let Entry::Fire(_) =
				self.grid[(x as i8 + dx) as usize][(y as i8 + dy) as usize]
			    {
				if rand::thread_rng().gen::<f32>() < self.fire_prob {
				    ret = Entry::Fire(FIRE_AGE);
				    break;
				}
			    }
			}
			ret
		    }

		    Entry::Fire(age) => {
			if age > self.fire_fw {
			    let randage = rand::thread_rng().gen_range(0, cmp::max(2, self.fire_fw));
			    Entry::Fire(age - randage)
			} else {
			    Entry::Burned(BURNT_AGE)
			}
		    }
		    Entry::Empty => {
			if rand::thread_rng().gen::<f32>() < self.tree_prob {
			    Entry::Tree
			} else {
			    Entry::Empty
			}
		    }
		    Entry::Burned(age) => {
			if age > self.burned_fw {
			    Entry::Burned(age - 1)
			} else if rand::thread_rng().gen::<f32>() < self.empty_prob {
			    Entry::Empty
			} else {
			    Entry::Burned(20)
			}
		    }
		    Entry::Water => Entry::Water,
		    _ => Entry::Empty,
		}
	    }
	}



	Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
	fn assign_rect(mb: &mut MeshBuilder, color: Color, counter: &mut (f32, f32)) {
	    mb.rectangle(
		DrawMode::fill(),
		Rect::new(counter.0, counter.1, 50.0, 50.0),
		color,
	    );
	    if counter.0 < SCREEN_WIDTH - ENTRY_WIDTH {
		counter.0 += ENTRY_WIDTH;
	    } else {
		counter.1 += ENTRY_HEIGHT;
		counter.0 = 0.0;
	    }
	}
	const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
	const GREEN: Color = Color::new(0.2, 0.5, 0.2, 1.0);
	const BROWN: Color = Color::new(0.46, 0.16, 0.16, 1.0);
	const GREY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
	const ORANGE: Color = Color::new(1.0, 0.45, 0.007, 1.0);
	const YELLOW: Color = Color::new(1.0, 0.8, 0.0, 1.0);
	const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
	graphics::clear(ctx, graphics::BLACK);
	let dst = nalgebra::Point2::new(0.0, 0.0);
	let mb = &mut MeshBuilder::new();
	let mut counter: (f32, f32) = (0.0, 0.0);
	for y in 0..100 {
	    for x in 0..100 {
		match self.grid[x][y] {
		    Entry::Fire(age) if age > (FIRE_AGE * 5 / 6) => {
			assign_rect(mb, YELLOW, &mut counter);
		    }
		    Entry::Fire(age) if age > (FIRE_AGE / 2) => {
			assign_rect(mb, ORANGE, &mut counter);
		    }
		    Entry::Fire(_) => {
			assign_rect(mb, RED, &mut counter);
		    }
		    Entry::Burned(_) => {
			assign_rect(mb, GREY, &mut counter);
		    }
		    Entry::Tree => {
			assign_rect(mb, GREEN, &mut counter);
		    }
		    Entry::Empty => {
			assign_rect(mb, BROWN, &mut counter);
		    }
		    Entry::Water => {
			assign_rect(mb, BLUE, &mut counter);
		    }
		    Entry::Edge => {
			assign_rect(mb, graphics::WHITE, &mut counter);
		    }
		}
	    }
	}
	let img = mb.build(ctx)?;
	graphics::draw(ctx, &img, (dst,))?;
	graphics::present(ctx)?;
	Ok(())
    }
}

fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = "Waldbrandsimulator".to_owned();
    //c.window_setup.icon = "/baum.png".to_owned(); Wenn Bild hinzugefügt
    c.window_mode.width = SCREEN_WIDTH;
    c.window_mode.height = SCREEN_HEIGHT;
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Firesim", "linus")
	.conf(c)
	.build()
	.unwrap();
    let state = &mut State::new(ctx);
    event::run(ctx, event_loop, state).unwrap();
}
