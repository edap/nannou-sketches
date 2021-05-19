use nannou::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Square {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub fn split_squares(x_val: f32, y_val: f32, squares: &mut Vec<Square>, prob: f32) {
    for i in (0..squares.len()).rev() {
        let square = squares[i].clone();
        if x_val > square.x && x_val < square.x + square.width {
            if random_range(0.0, 1.0) > prob {
                split_on_x(i, squares, x_val);
            }
        }
    }

    for i in (0..squares.len()).rev() {
        let square = squares[i].clone();
        if y_val > square.y && (y_val < square.y + square.height) {
            if random_range(0.0, 1.0) > prob {
                split_on_y(i, squares, y_val);
            }
        }
    }
}

fn split_on_x(square_index: usize, squares: &mut Vec<Square>, split_at: f32) {
    let square = &squares[square_index];
    let square_a = Square {
        x: square.x,
        y: square.y,
        width: square.width - (square.width - split_at + square.x),
        height: square.height,
    };
    let square_b = Square {
        x: split_at,
        y: square.y,
        width: square.width - split_at + square.x,
        height: square.height,
    };
    let copy_squares = squares.clone();
    squares.clear();

    for i in 0..copy_squares.len() {
        if i == square_index {
            squares.push(square_a);
            squares.push(square_b);
        } else {
            squares.push(copy_squares[i]);
        }
    }
}

fn split_on_y(square_index: usize, squares: &mut Vec<Square>, split_at: f32) {
    let square = &squares[square_index];
    let square_a = Square {
        x: square.x,
        y: square.y,
        width: square.width,
        height: square.height - (square.height - split_at + square.y),
    };
    let square_b = Square {
        x: square.x,
        y: split_at,
        width: square.width,
        height: square.height - split_at + square.y,
    };
    // make a copy
    let copy_squares = squares.clone();
    squares.clear();
    for i in 0..copy_squares.len() {
        if i == square_index {
            squares.push(square_a);
            squares.push(square_b);
        } else {
            squares.push(copy_squares[i]);
        }
    }
}
