enum State {
    Empty,
    Tree,
    Fire,
}

struct Config {
    fire_prob: f32,
    neighbours: [(i8, i8); 8],
    finished: bool,
}

fn transition(grid: &mut [[State; 100]; 100], config: &mut Config) {
    config.finished = true;
    for y in 1..99 {
		for x in 1..99 {
		    if grid[x][y] == State::Empty {
				continue;
		    }
		    config.finished = false; // wird in jeder Schleife zugewiesen
		    for (dx, dy) in config.neighbours.iter() {
				grid[x][y] = match grid[x + dx][y + dy] {
				    State::Fire => {
						if random() < config.fire_prob {
						    // nicht implementiert
						    State::Fire
						} else {
						    State::Tree
						}
				    }
				    _ => State::Tree,
				}
		    }
		}
    }
}

fn main() {
    let mut forrest = [[State::Empty; 100]; 100];
    let mut config = Config {
		fire_prob: 0.5,
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
    };
    for y in 0..100 {
		for x in 0..100 {
		    forrest[x][y] = random(); // nicht implementiert
		}
    }
    while !config.finished {
		render(forrest); // nicht implementiert
		transition(&mut forrest, &mut config);
    }
}
