
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
    //c.window_setup.icon = "/baum.png".to_owned();
    c.window_mode.width = SCREEN_WIDTH;
    c.window_mode.height = SCREEN_HEIGHT;
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Firesim", "linus")
	.conf(c)
	.build()
	.unwrap();
    let state = &mut State::new(ctx);
    event::run(ctx, event_loop, state).unwrap();
}
