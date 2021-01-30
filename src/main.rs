use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

type Grid = Vec<Vec<bool>>;
#[derive(Debug)]
struct Board {
    rows: usize,
    cols: usize,
    grid: Grid,
}

impl Board {
    fn new(rows: usize, cols: usize) -> Board {
        Board {
            rows,
            cols,
            grid: vec![vec![false; rows]; cols],
        }
    }

    fn get_index(&self, row: usize, col: usize) -> bool {
        self.grid[row][col]
    }

    fn set_index(&mut self, row: usize, col: usize, is_life: bool) {
        self.grid[row][col] = is_life;
    }

    fn get_neighbor_count(&self, row: usize, col: usize) -> usize {
        let mut count: usize = 0;

        // NW
        if row > 0 && col > 0 && self.get_index(row - 1, col - 1) {
            count += 1;
        }
        // N
        if row > 0 && self.get_index(row - 1, col) {
            count += 1;
        }
        // NE
        if row > 0 && col < self.cols - 1 && self.get_index(row - 1, col + 1) {
            count += 1;
        }
        // W
        if col > 0 && self.get_index(row, col - 1) {
            count += 1;
        }
        // E
        if col < self.cols - 1 && self.get_index(row, col + 1) {
            count += 1;
        }
        // SW
        if row < self.rows - 1 && col > 0 && self.get_index(row + 1, col - 1) {
            count += 1;
        }
        // S
        if row < self.rows - 1 && self.get_index(row + 1, col) {
            count += 1;
        }
        // SE
        if row < self.rows - 1 && col < self.cols - 1 && self.get_index(row + 1, col + 1) {
            count += 1;
        }
        count
    }
}

#[derive(Debug)]
struct Game {
    board: Board,
}

impl Game {
    fn new(board: Board) -> Game {
        Game { board }
    }

    fn death(grid: &mut Grid, row: usize, col: usize) {
        grid[row][col] = false;
    }

    fn life(grid: &mut Grid, row: usize, col: usize) {
        grid[row][col] = true;
    }

    // Rules that govern the game of life
    fn solitude(grid: &mut Grid, row: usize, col: usize) {
        Game::death(grid, row, col);
    }

    fn overpopulation(grid: &mut Grid, row: usize, col: usize) {
        Game::death(grid, row, col);
    }
    // A balanced function
    fn balance(_grid: &mut Grid) {}

    fn populate(grid: &mut Grid, row: usize, col: usize) {
        Game::life(grid, row, col);
    }

    pub fn pass_year(&mut self) {
        let mut temp_grid = self.board.grid.clone();
        for row in 0..self.board.rows {
            for col in 0..self.board.cols {
                let is_alive = self.board.get_index(row, col);
                let neighbor_count = self.board.get_neighbor_count(row, col);

                if is_alive {
                    match neighbor_count {
                        0 | 1 => Game::solitude(&mut temp_grid, row, col),
                        2 | 3 => Game::balance(&mut temp_grid),
                        count if count >= 4 => Game::overpopulation(&mut temp_grid, row, col),
                        _ => {}
                    }
                } else if let 3 = neighbor_count {
                    Game::populate(&mut temp_grid, row, col);
                }
            }
        }
        self.board.grid = temp_grid;
    }
}

fn main() {
    const ROWS: usize = 30;
    const COLS: usize = 30;
    const SCALE: u32 = 30;
    const WIDTH: u32 = (COLS as u32) * SCALE;
    const HEIGHT: u32 = (ROWS as u32) * SCALE;
    let board = Board::new(ROWS, COLS);
    let mut game = Game::new(board);

    // game.board.set_index(0, 0, true);
    game.board.set_index(14, 14, true);
    game.board.set_index(14, 15, true);
    game.board.set_index(14, 16, true);
    game.board.set_index(15, 14, true);
    game.board.set_index(15, 16, true);
    game.board.set_index(16, 14, true);
    game.board.set_index(16, 15, true);
    game.board.set_index(16, 16, true);
    // game.board.set_index(0, 2, true);
    // game.board.set_index(2, 1, true);

    // Init SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8 Emulator", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let tex_creator = canvas.texture_creator();
    canvas.set_draw_color(Color::GREEN);

    'running: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Create a texture to render in SDL
        let mut texture = tex_creator
            .create_texture_target(tex_creator.default_pixel_format(), WIDTH, HEIGHT)
            .unwrap();

        canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.set_draw_color(Color::BLACK);
                texture_canvas.clear();
                texture_canvas.set_draw_color(Color::YELLOW);
                for row in 0..game.board.rows {
                    for col in 0..game.board.cols {
                        if game.board.get_index(row, col) {
                            texture_canvas
                                .fill_rect(Rect::new(
                                    ((col as u32) * SCALE) as i32,
                                    ((row as u32) * SCALE) as i32,
                                    SCALE,
                                    SCALE,
                                ))
                                .unwrap();
                        }
                    }
                }
            })
            .unwrap();

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        game.pass_year();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32));
    }
}
