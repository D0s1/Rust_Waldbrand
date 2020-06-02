use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics::{self, Color, DrawMode, MeshBuilder, Rect};
use ggez::nalgebra;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use rand::{Rand, Rng};


const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 1000.0;
const ENTRY_WIDTH: f32 = SCREEN_WIDTH / 100.0;
const ENTRY_HEIGHT: f32 = SCREEN_HEIGHT / 100.0;
const FIRE_AGE: u8 = 200;

#[derive(Debug, Copy, Clone)]
enum Entry {
    Empty,
    Tree,
    Fire(u8),
}

impl Rand for Entry {
    fn rand<R: Rng>(rng: &mut R) -> Self {
	match rng.gen::<f32>() {
	    i if i < 0.1 => Entry::Empty,
	    _ =>  Entry::Tree,
	    
	}
    }
}

struct State {
    grid: [[Entry; 100]; 100],
    fire_prob: f32,
    spawn_tree_prob: f32,
    neighbours: [(i8, i8); 8],
    finished: bool,
}

impl State {
    fn new(ctx: &mut Context) -> State {
	let mut rng = rand::thread_rng();
	let mut grid = [[Entry::Empty; 100]; 100];
	for y in 1..99 {
	    for x in 1..99 {
		grid[y][x] = rng.gen();
	    }
	}
	let rand_x = rng.gen_range(1, 99);
	let rand_y = rng.gen_range(1, 99);
	grid [rand_x] [rand_y] = Entry::Fire(FIRE_AGE);
	State {
	    grid,
	    fire_prob: 0.01,
	    spawn_tree_prob: 0.01,
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
	    finished: false,
	}
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
	
	if timer::check_update_time(ctx, 20) {
	    for y in 1..99 {
		for x in 1..99 {
		    if let Entry::Empty = self.grid[x][y] {
				if rand::thread_rng().gen::<f32>() < self.spawn_tree_prob {
					self.grid[x][y] = Entry::Tree
				}
		    }
		    self.grid[x][y] = match self.grid[x][y] {
			Entry::Tree => {
			    let mut fire_p = false;
			    for (dx, dy) in self.neighbours.iter() {
				if let Entry::Fire(_) =
				    self.grid[(x as i8 + dx) as usize][(y as i8 + dy) as usize]
				{	
					
				    if rand::thread_rng().gen::<f32>() < self.fire_prob {
					fire_p = true;
					break;
				    }
				}
			    }
			    if fire_p {
				Entry::Fire(FIRE_AGE)
			    } else {
				Entry::Tree
			    }
			}

			Entry::Fire(age) => {
			    if age >= 1 {
				Entry::Fire(age - 1)
			    } else {
				Entry::Empty
			    }
			}
			_ => Entry::Empty,
		    }
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
	const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
	graphics::clear(ctx, graphics::BLACK);
	let dst = nalgebra::Point2::new(0.0, 0.0);
	let mb = &mut MeshBuilder::new();
	// let (s_width, s_height) = graphics::size(ctx);
	// println!("{} {}", s_width, s_height);
	let mut counter: (f32, f32) = (0.0, 0.0);
	for y in 0..100 {
	    for x in 0..100 {
		match self.grid[x][y] {
		    Entry::Fire(_) => {
			assign_rect(mb, RED, &mut counter);
		    }
		    Entry::Tree => {
			assign_rect(mb, GREEN, &mut counter);
		    }
		    Entry::Empty => {
			assign_rect(mb, graphics::WHITE, &mut counter);
		    }
		}
	    }
	}
	let rect = mb.build(ctx)?;
	graphics::draw(ctx, &rect, (dst,))?;
	graphics::present(ctx)?;
	Ok(())
    }
}

fn main() {
    let mut c = conf::Conf::new();
    c.window_mode.width = SCREEN_WIDTH;
    c.window_mode.height = SCREEN_HEIGHT;
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Firesim", "linus")
	.conf(c)
	.build()
	.unwrap();
    let state = &mut State::new(ctx);
    event::run(ctx, event_loop, state).unwrap();
}
