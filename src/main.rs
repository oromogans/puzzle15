use macroquad::prelude::*;

const MOVES_FROM_START: i8 = 100;

#[derive(Debug)]
struct Frame {
    tiles: [[usize; 4]; 4],
    zero: (usize, usize),
}

impl Frame {
    fn new() -> Frame {
        let mut number = 1;
        let mut tiles = [[0, 0, 0, 0]; 4];
        for i in 0..4 {
            for j in 0..4 {
                if number == 16 {
                    number = 0
                };
                tiles[i][j] =number;
                number += 1;
            }
        }
        Frame {
            tiles,
            zero: (3, 3),
        }
    }

    fn draw(&self) {
        let cellcolor;
        let y_centers = [0.125, 0.375, 0.625, 0.875].map(|x| x * screen_height());
        let x_centers = [0.125, 0.375, 0.625, 0.875].map(|x| x * screen_width());
        {
            if is_done(self) {
                cellcolor = DARKGREEN;
            } else {
                cellcolor = BLACK;
            }
            for (row_index, y_c) in y_centers.iter().enumerate() {
                for (col_index, x_c) in x_centers.iter().enumerate() {
                    let n = self.tiles[row_index][col_index];
                    if n != 0 {
                        draw_text(format!("{}", n).as_str(), *x_c, *y_c, 50., cellcolor);
                    }
                }
            }
        }
    }
}

fn get_valid_moves(zero: (usize, usize)) -> Vec<(usize, usize)> {
    let zero_int: (i8, i8) = (zero.0 as i8, zero.1 as i8);
    let mut valid_moves: Vec<(usize, usize)> = vec!();
    let up = (zero_int.0 - 1, zero_int.1);
    let down = (zero_int.0 + 1, zero_int.1);
    let left = (zero_int.0, zero_int.1 - 1);
    let right = (zero_int.0, zero_int.1 + 1);

    for m in [up, down, left, right] {
        if m.0 >= 0 && m.1 >= 0 && m.0 < 4 && m.1 < 4 {
            let valid_move = (m.0 as usize, m.1 as usize);
            valid_moves.push(valid_move);
        }
    }
    valid_moves
}

fn make_random_move(f: Frame, times: i8) -> Frame {
    let mut updated_tiles = f;
    for _ in 0..times {
        let cross = get_valid_moves(updated_tiles.zero);
        let index_move = rand::gen_range(0, cross.len());
        updated_tiles = swap_zero_with_tile(&updated_tiles, cross[index_move]);
    }
    updated_tiles
}

fn swap_zero_with_tile(f: &Frame, tile_pos: (usize, usize)) -> Frame {
    let mut frame_copy = f.tiles;
    frame_copy[f.zero.0][f.zero.1] = frame_copy[tile_pos.0][tile_pos.1];
    frame_copy[tile_pos.0][tile_pos.1] = 0;
    Frame {
        tiles: frame_copy,
        zero: tile_pos,
    }
}

fn is_valid_move(f: &Frame, move_to_check: &(usize, usize)) -> bool {
    let valid_moves = get_valid_moves(f.zero);
    if valid_moves.contains(move_to_check) {
        return true;
    }
    false
}

fn is_done(f: &Frame) -> bool {
    let mut shouldbe = 1;
    for i in 0..4 {
        for j in 0..4 {
            if f.tiles[i][j] != shouldbe {
                return false;
            }
            if shouldbe < 15 {
                shouldbe += 1;
            } else {
                shouldbe = 0;
            }
        }
    }
    true
}

fn mouse_coordiante_to_field_coordinate(coord: (f32, f32)) -> (usize, usize) {
    let mut x = 0;
    let mut y = 0;

    let y_points = [0.25, 0.5, 0.75, 1.0].map(|x| x * screen_height());
    let x_points = [0.25, 0.5, 0.75, 1.0].map(|x| x * screen_width());

    for (index_y, y_p) in y_points.iter().enumerate() {
        if coord.1 < *y_p {
            x = index_y;
            break;
        };
    }
    for (index_x, x_p) in x_points.iter().enumerate() {
        if coord.0 < *x_p {
            y = index_x;
            break;
        };
    }
    (x, y)
}

fn draw_grid() {
    let y_points = [0.25, 0.5, 0.75].map(|x| x * screen_height());
    let x_points = [0.25, 0.5, 0.75].map(|x| x * screen_width());

    for y_p in y_points {
        draw_line(0.0, y_p, screen_width(), y_p, 1.0, BLACK);
    }
    for x_p in x_points {
        draw_line(x_p, 0.0, x_p, screen_height(), 1.0, BLACK);
    }
}

#[macroquad::main("Puzzle 15")]
async fn main() {
    let init_frame = Frame::new();
    let mut current_frame = make_random_move(init_frame, MOVES_FROM_START);
    loop {
        clear_background(LIGHTGRAY);
        if is_mouse_button_released(MouseButton::Left) {
            let pos = mouse_position();
            let move_coordinates = mouse_coordiante_to_field_coordinate(pos);

            if is_valid_move(&current_frame, &move_coordinates ) {
                current_frame = swap_zero_with_tile(&current_frame, move_coordinates);
            }
        }

        draw_grid();
        current_frame.draw();
        next_frame().await
    }
}
