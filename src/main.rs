use std::io::stdin;
use macroquad::prelude::*;

#[derive(Debug)]
struct Field {
    field: [[usize; 4]; 4],
    zero: (usize, usize),
}

impl Field {
    fn new() -> Field {
        let mut value = 0;
        let mut field = [[0, 0, 0, 0]; 4];
        for i in 0..4 {
            for j in 0..4 {
                field[i][j] = value;
                value += 1;
            }
        }
        Field {
            field,
            zero: (0, 0),
        }
    }

    fn draw(&self) {
        for i in 0..4 {
            print!("{} | ", 4 - i);
            for j in 0..4 {
                if self.field[i][j] > 10 {
                    print!("{} ", self.field[i][j]);
                } else {
                    print!("{}  ", self.field[i][j]);
                }
            }
            println!();
        }
        println!("- |------------");
        println!("- |  a  b  c  d");
    }
}

fn get_valid_moves(zero: (usize, usize)) -> Vec<(usize, usize)> {
    let zero_int: (i8, i8) = (zero.0 as i8, zero.1 as i8);
    let mut valid_moves: Vec<(usize, usize)> = [].to_vec();
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

fn make_random_move(f: Field, times: i8) -> Field {
    let mut updated_field = f;
    for _ in 0..times {
        let cross = get_valid_moves(updated_field.zero);
        let index_move = rand::gen_range(0, cross.len());
        updated_field = swap_zero(&updated_field, cross[index_move]);
    }
    updated_field
}

fn swap_zero(f: &Field, pos: (usize, usize)) -> Field {
    let mut field_copy = f.field;
    field_copy[f.zero.0][f.zero.1] = field_copy[pos.0][pos.1];
    field_copy[pos.0][pos.1] = 0;
    Field {
        field: field_copy,
        zero: pos,
    }
}

fn user_input_to_coord(ui: &str) -> Option<(usize, usize)> {
    // Input should look like "a2"
    let letter = ui.chars().next().unwrap();
    let number = ui.chars().nth(1).unwrap();

    let row: usize;
    let col: usize;

    match letter.to_string().as_str() {
        "a" => col = 0,
        "b" => col = 1,
        "c" => col = 2,
        "d" => col = 3,
        _default => return None,
    }

    match number.to_string().as_str() {
        "1" => row = 3,
        "2" => row = 2,
        "3" => row = 1,
        "4" => row = 0,
        _default => return None,
    }
    Some((row, col))
}

fn is_valid_move(field: &Field, mv: &(usize, usize)) -> bool {
    let valid_moves = get_valid_moves(field.zero);
    if valid_moves.contains(mv) {
        return true;
    }
    false
}

fn is_done(field: &Field) -> bool {
    let mut shouldbe = 0;
    for i in 0..4 {
        for j in 0..4 {
            if field.field[i][j] != shouldbe {
                return false;
            }
            shouldbe += 1;
        }
    }
    true
}

#[macroquad::main("Puzzle 15")]
async fn main() {
    println!("Hee");
    let y_points = [0.25, 0.5, 0.75].map(|x|{x*screen_height()});
    let x_points = [0.25, 0.5, 0.75].map(|x|{x*screen_width()});

    let y_centers = [0.125, 0.375, 0.625, 0.875].map(|x|{x*screen_height()});
    let x_centers = [0.125, 0.375, 0.625, 0.875].map(|x|{x*screen_width()});
    loop {
        clear_background(LIGHTGRAY);
        for y_p in y_points {
            draw_line(
                0.0,
                y_p,
                screen_width(),
                y_p,
                1.0,
                BLACK,
                );

        }
        for x_p in x_points {
            draw_line(
                x_p,
                0.0,
                x_p,
                screen_height(),
                1.0,
                BLACK
            );

        for x_c in x_centers{
            for y_c in y_centers {
                draw_text(
                    format!("{}", 0).as_str(),
                    x_c,
                    y_c,
                    20.,
                    DARKGRAY,
                    );
            }
        }
        }
        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_uinput_convert() {
        let uinp = "a1".to_string();
        let result = user_input_to_coord(&uinp);
        assert_eq!(result.unwrap(), (3, 0));
    }

    #[test]
    fn test_shuffle() {
        let field = Field::new();
        let upd = make_random_move(field, 60);
        println!("{:?}", upd.field);
        assert_eq!(upd.field.len(), 4)
    }

    #[test]
    fn test_get_valid_moves_from_zero() {
        let expected: Vec<(usize, usize)> = [(1, 0), (0, 1)].to_vec();
        let expected_2: Vec<(usize, usize)> = [(2, 3), (3, 2)].to_vec();
        assert_eq!(get_valid_moves((0, 0)), expected);
        assert_eq!(get_valid_moves((3, 3)), expected_2);
    }

    #[test]
    fn test_swap() {
        let field = Field::new();
        let updated = swap_zero(&field, (1, 0));
        assert_eq!(updated.zero, (1, 0));
        assert_eq!(updated.field[1][0], 0);
        assert_eq!(updated.field[0][0], 4);
    }
}
