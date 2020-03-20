use rand::{self, Rng};
use std::f32;

// 落ちてくるブロックの色
const COLORS: &'static [Color] = &[
    Color::Black,
    Color::Red,
    Color::Green,
    Color::Blue,
    Color::Yellow,
    Color::Cyan,
    Color::Magenta,
    Color::White,
];

// 落ちてくるブロックの形
const BLOCKS: &'static [&'static [(i32, i32)]] = &[
    &[(0, 0), (0, 1), (1, 0), (1, 1)],
    &[(0, 0), (0, 1), (0, 2), (1, 1), (2, 1)],
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    &[(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)],
    &[(0, 0), (0, 1), (0, 2), (0, 3), (1, 0)],
    &[(0, 0), (0, 1), (1, 1), (1, 2)],
    &[(1, 0), (1, 1), (0, 1), (0, 2)],
    &[(0, 1), (0, 1), (0, 2), (1, 1)],
];

// 操作するブロックとブロックを配置するフィールドを状態として持つ
struct Tetris {
    block: Block,
    field: [[Color; 10]; 20],
}

impl Tetris {
    fn new() -> Tetris {
        Tetris {
            block: Block::rand(),
            field: [[Color::Black; 10]; 20],
        }
    }

    fn control(&mut self, op: Control) {
        let pre = self.block.blocks.clone();
        match op {
            Control::Down => self.block.down(),
            Control::Left => self.block.left(),
            Control::Right => self.block.right(),
            Control::Rotate => self.block.rotate(),
        }

        let ly = self.field.len() as i32;
        let lx = self.field[0].len() as i32;
        let exists = self.block.blocks.iter().all(|&(y, x)| {
            0 <= y
                && y < ly
                && 0 <= x
                && x < lx
                && (self.field[y as usize][x as usize] == Color::Black)
        });

        if !exists {
            self.block.blocks = pre;
        }
    }

    fn delete(&mut self) {
        for y in 0..self.field.len() {
            if self.field[y].iter().all(|c| *c == Color::Black) {
                continue;
            }
            for x in 0..self.field[y].len() {
                let mut yy = y;
                for yyy in (0..y - 1).rev() {
                    self.field[yy][x] = self.field[yyy][x];
                    yy -= 1;
                }
            }
        }
    }

    fn fall(&mut self) {
        let blocks = self.block.blocks.clone();
        self.control(Control::Down);

        let mut not_moved = true;
        for i in 0..self.block.blocks.len() {
            if self.block.blocks[i] != blocks[i] {
                not_moved = false;
                break;
            }
        }

        if not_moved {
            {
                let ref bs = self.block.blocks;
                for &(y, x) in bs {
                    self.field[y as usize][x as usize] = self.block.color;
                }
            }
            self.block = Block::rand();
            self.delete();
        }
    }
}

// ひとかたまり同じ色として扱う
struct Block {
    color: Color,
    blocks: Vec<(i32, i32)>,
}

#[derive(PartialEq, Copy, Clone)]
enum Color {
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
}

#[derive(PartialEq)]
enum Control {
    Down,
    Left,
    Right,
    Rotate,
}

impl Block {
    fn new(c: Color, b: Vec<(i32, i32)>) -> Block {
        Block {
            color: c,
            blocks: b,
        }
    }

    fn rand() -> Block {
        let mut rng = rand::thread_rng();
        let colors_range = rng.gen_range(0, COLORS.len());
        let blocks_range = rng.gen_range(0, BLOCKS.len());
        Block {
            color: COLORS[colors_range],
            blocks: BLOCKS[blocks_range].to_vec(),
        }
    }

    fn down(&mut self) {
        for c in self.blocks.iter_mut() {
            c.0 += 1;
        }
    }

    fn left(&mut self) {
        for c in self.blocks.iter_mut() {
            c.1 -= 1;
        }
    }

    fn right(&mut self) {
        for c in self.blocks.iter_mut() {
            c.1 += 1;
        }
    }

    fn rotate(&mut self) {
        let r: f32 = f32::consts::PI / 2.0;
        let cy: f32 =
            (self.blocks.iter().map(|i| i.0).sum::<i32>() as f32) / (self.blocks.len() as f32);
        let cx: f32 =
            (self.blocks.iter().map(|i| i.1).sum::<i32>() as f32) / (self.blocks.len() as f32);

        for c in self.blocks.iter_mut() {
            let (y, x) = *c;
            let y = f32::from(y as i16);
            let x = f32::from(x as i16);
            *c = (
                (cy + (x - cx) * r.sin() + (y - cy) * r.cos()).round() as i32,
                (cx + (x - cx) * r.cos() + (y - cy) * r.sin()).round() as i32,
            )
        }
    }
}

fn main() {
    println!("Hello, world!");
}
