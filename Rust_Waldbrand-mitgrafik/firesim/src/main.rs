use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics::{self, Color, DrawMode, MeshBuilder, Rect};
use ggez::nalgebra;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use rand::{Rand, Rng};
use ggez::input::keyboard;
use ggez::input::mouse;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use std::{thread, time};

const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 1000.0;
const ENTRY_WIDTH: f32 = SCREEN_WIDTH / 100.0;
const ENTRY_HEIGHT: f32 = SCREEN_HEIGHT / 100.0;
const FIRE_AGE: u32 = 300;
const BURNT_AGE: u32 = 650;

#[derive(Debug, Copy, Clone)]
enum Entry {
    Edge,
    Empty,
    Tree,
	Water,
    Fire(u32),
    Burnt(u32),
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
    fire_prob: f32,
	pause: bool,
    spawn_tree_prob: f32,
    empty_prob: f32,
    neighbours: [(i8, i8); 8],
}

impl State {
    fn new(ctx: &mut Context) -> State {
	let mut rng = rand::thread_rng();
	let mut grid = [[Entry::Edge; 100]; 100];
	for y in 1..99 {
	    for x in 1..99 {
		grid[y][x] = rng.gen();
	    }
	}
	let rand_x = rng.gen_range(1, 99);
	let rand_y = rng.gen_range(1, 99);
	grid[rand_x][rand_y] = Entry::Fire(FIRE_AGE);
	State {
	    grid,
	    fire_prob: 0.0025,
		pause: false,
	    spawn_tree_prob: 0.000005,
	    empty_prob: 0.0002,
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
	}
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
	
	if timer::check_update_time(ctx, 60) {
	
	let mut firemul= 2 as u32; //Feuer Ausbreitungsfaktor
	let mut fw = 20 as u32; //Fast-fw fuer FireAge
	let mut bfw=1 as u32; //Burnt Fast Forward
	
	if keyboard::is_key_pressed(ctx, KeyCode::P){ 
	if self.pause{self.pause = false;}
	else {self.pause = true}
	let wait = time::Duration::from_millis(500);
	let now = time::Instant::now();

	thread::sleep(wait);

	assert!(now.elapsed() >= wait);}
	

	
	
	if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
				// Hier Variablen anpassen für FW
                self.fire_prob = 0.05;   //Feuer-ausbreitgeschwindigkeit
				self.spawn_tree_prob= 0.0005; // Baueme spawnen
				bfw=550;   // 650 - bfw = Zeit bis Empty spawnen kann
				self.empty_prob = 0.001;  //Wahrscheinlichkeit das Empty spawnen kann pro tick nach Zeit
				firemul = fw  //Wird fuer Abfragen benoetigt, zum veraendern fw oben bearbeiten!!
            }
			else {self.fire_prob = 0.0025;
			self.spawn_tree_prob= 0.000005;
			self.empty_prob=0.0002;
			bfw=1;
			firemul=2}
	if keyboard::is_mod_active(ctx, KeyMods::ALT){
	let mouse_position = ggez::input::mouse::position(ctx);
	let x = ((mouse_position.x / 10.0)) as usize;
	let y = ((mouse_position.y / 10.0)) as usize;
	if (x) >= 100 || (y) >=100 || x == 0 || y == 0 || x == 99 || y == 99
	{}
	else{
	self.grid[x][y] = match self.grid[x][y] {
			_ => {Entry::Fire(FIRE_AGE)}}
			}
			}
	if keyboard::is_mod_active(ctx, KeyMods::CTRL){
	let mouse_position = ggez::input::mouse::position(ctx);
	let x = ((mouse_position.x / 10.0)) as usize;
	let y = ((mouse_position.y / 10.0)) as usize;
	if (x) >= 100 || (y) >=100 || x == 0 || y == 0 || x == 99 || y == 99
	{}
	else{
	self.grid[x][y] = match self.grid[x][y] {
			_ => {Entry::Water}}
			}
			}
	if keyboard::is_key_pressed(ctx, KeyCode::R){ {
	
	}
	for y in 1..99 {
	    for x in 1..99 {
		let mut rng = rand::thread_rng();
		let mut i = rand::thread_rng().gen::<f32>();
	    if i < 0.1 {
		self.grid[x][y] = match self.grid[x][y] {
		_ => {Entry::Empty}}}
		else {
		self.grid[x][y] = match self.grid[x][y] {
		_ => {Entry::Tree}}
	    
	}
	    }
			}
			
			}
	if self.pause==false {
	    for y in 1..99 {
		for x in 1..99 {
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
			    if age >= (firemul-1) {
				let randagesub = rand::thread_rng().gen_range(0,firemul);
				Entry::Fire(age - randagesub)
			    } else {
				Entry::Burnt(BURNT_AGE)
			    }
			}
			Entry::Empty => {
			    if rand::thread_rng().gen::<f32>() < self.spawn_tree_prob {
				Entry::Tree
			    } else {
				Entry::Empty
			    }
			}
			Entry::Burnt(age) => {
			    if age >= bfw {
				Entry::Burnt(age - 1)
			    } else if rand::thread_rng().gen::<f32>() < self.empty_prob {
				Entry::Empty
			    } else {
				Entry::Burnt(20)
				}
			}
			Entry::Water =>{
			Entry::Water}
			_ => Entry::Empty,
		    }
			
		}
	    }
	}}
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
	const BROWN: Color = Color::new(0.46,0.16,0.16,1.0);
	const GREY: Color = Color::new(0.5,0.5,0.5,1.0);
	const ORANGE: Color = Color::new(1.0,0.45,0.007,1.0);
	const YELLOW: Color = Color::new(1.0,0.8,0.0,1.0);
	const BLUE: Color = Color::new(0.0,0.0,1.0,1.0);
	graphics::clear(ctx, graphics::BLACK);
	let dst = nalgebra::Point2::new(0.0, 0.0);
	let mb = &mut MeshBuilder::new();
	let mut counter: (f32, f32) = (0.0, 0.0);
	for y in 0..100 {
	    for x in 0..100 {
		match self.grid[x][y] {
		    Entry::Fire(age) => {
				if age > FIRE_AGE/2 + FIRE_AGE/3 {
						assign_rect(mb, YELLOW, &mut counter);
					} else if age > FIRE_AGE/2 {
						assign_rect(mb, ORANGE, &mut counter);
					}else {
						assign_rect(mb, RED, &mut counter);
					}
		    }
		    Entry::Burnt(_) => {
			assign_rect(mb, GREY, &mut counter);
		    }
		    Entry::Tree => {
			assign_rect(mb, GREEN, &mut counter);
		    }
		    Entry::Empty => {
			assign_rect(mb, BROWN, &mut counter);
		    }
		    Entry::Edge => {
			assign_rect(mb, graphics::WHITE, &mut counter);
		    }
			Entry::Water => {
			assign_rect(mb, BLUE, &mut counter);
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
