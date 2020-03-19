use rand::{self, Rng};

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
}

fn main() {
    println!("Hello, world!");
}
