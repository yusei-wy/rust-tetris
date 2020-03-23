#[macro_use]
extern crate glium;
use glium::{glutin, index, vertex};
use glium::{Display, Program, Surface};

extern crate rust_tetris;
use rust_tetris::tetris::{Color, Tetris};

use std::sync::{Arc, Mutex};
use std::{f32, thread, time};

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
}
implement_vertex!(Vertex, pos, color);

const VERTEX_SHADER: &'static str = r#"
#version 400

in vec2 pos;
in vec4 color;
out vec4 v_color;

void main() {
    v_color = color;
    gl_position = vec4(pos, 0, 1);
}"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 400

in vec4 v_color;
out vec4 f_color;

void main() {
    f_color = v_color;
}
"#;

fn tetris_to_vertex(tetris: &Tetris) -> Vec<Vertex> {
    let mut vs = vec![];
    let mut y: f32 = 0.80;
    let mut iy: usize = 0;

    while y >= -0.885 {
        let mut x: f32 = -0.375;
        let mut ix: usize = 0;

        while x <= 0.46 {
            if tetris
                .block
                .blocks
                .iter()
                .any(|&(yy, xx)| iy as i32 == yy && ix as i32 == xx)
            {
                vs.push(Vertex {
                    pos: [x, y],
                    color: color_to_rgba(tetris.block.color),
                });
            } else {
                vs.push(Vertex {
                    pos: [x, y],
                    color: color_to_rgba(tetris.field[iy][ix]),
                });
            }

            x += 0.086;
            ix += 1;
        }

        y -= 0.085;
        iy += 1;
    }

    vs
}

fn color_to_rgba(c: Color) -> [f32; 4] {
    match c {
        Color::Black => [0.0, 0.0, 0.0, 0.0],
        Color::Red => [0.5, 0.0, 0.0, 0.5],
        Color::Green => [0.0, 0.5, 0.0, 0.5],
        Color::Blue => [0.0, 0.0, 0.5, 0.5],
        Color::Yellow => [0.0, 0.5, 0.0, 0.5],
        Color::Cyan => [0.0, 0.5, 0.5, 0.5],
        Color::Magenta => [0.5, 0.0, 0.5, 0.5],
        Color::White => [0.5, 0.5, 0.5, 0.5],
    }
}

fn main() {}
