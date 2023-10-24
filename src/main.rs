use std::backtrace::BacktraceStatus::Disabled;
//use macroquad::miniquad::CursorIcon::Default as DefaultCursor;
use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};
use strum::{EnumCount, IntoEnumIterator,};
use strum_macros::{EnumCount , EnumIter, FromRepr};
use core::default::Default;

// #[macro_use]
// extern crate num_derive;


const SIZE:f32 = 100.0;
const DIM:usize = 4; //Dimension of the grid

#[derive(Debug,Default,Clone,Copy,Hash,EnumCount,EnumIter,FromRepr)]
enum TileTypeIndex {
    #[default]
    None = 0,
    Blank = 1,
    HLine = 2,
    VLine = 3,
    LBL = 4,
    LBR = 5,
    LTL = 6,
    LTR = 7,
    TTop =8,
    TRight = 9,
    TBottom = 10,
    TLeft = 11,
}
impl TileTypeIndex {
    fn connections(&self) -> [bool; 4] {
        //ok this is objectively bad code, i know, but i didn't write it, copilot did and i ain't changing it
        #[allow(unused_assignments)]
            let mut connections = [false; 4];
        match self {
            TileTypeIndex::None => {
                panic!("⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠛⠛⠛⠛⠿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠉⠻⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⠋⠈⠀⠀⠀⠀⠐⠺⣖⢄⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⡏⢀⡆⠀⠀⠀⢋⣭⣽⡚⢮⣲⠆⠀⠀⠀⠀⠀⠀⢹⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⡇⡼⠀⠀⠀⠀⠈⠻⣅⣨⠇⠈⠀⠰⣀⣀⣀⡀⠀⢸⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⡇⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣟⢷⣶⠶⣃⢀⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⡅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⠀⠈⠓⠚⢸⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⢀⡠⠀⡄⣀⠀⠀⠀⢻⠀⠀⠀⣠⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠐⠉⠀⠀⠙⠉⠀⠠⡶⣸⠁⠀⣠⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣦⡆⠀⠐⠒⠢⢤⣀⡰⠁⠇⠈⠘⢶⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠠⣄⣉⣙⡉⠓⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⣀⣀⠀⣀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n");   //compatablity = [true;4];
            },
            TileTypeIndex::Blank => {
                connections = [false; 4];
            },
            TileTypeIndex::HLine => {
                connections = [true, false, true, false];
            },
            TileTypeIndex::VLine => {
                connections = [false, true, false, true];
            },
            TileTypeIndex::LBL => {
                connections = [false, false, true, true];
            },
            TileTypeIndex::LBR => {
                connections = [false, true, true, false];
            },
            TileTypeIndex::LTL => {
                connections = [true, false, false, true];
            },
            TileTypeIndex::LTR => {
                connections = [true, true, false, false];
            },
            TileTypeIndex::TTop => {
                connections = [true, true, false, true];
            },
            TileTypeIndex::TRight => {
                connections = [true, true, true, false];
            },
            TileTypeIndex::TBottom => {
                connections = [false, true, true, true];
            },
            TileTypeIndex::TLeft => {
                connections = [true, false, true, true];
            },
        }
        return connections;
    }
    fn is_compatible_with(&self, tile2: TileTypeIndex, side: Side) -> bool {
        return if self.connections()[side as usize] == tile2.connections()[side.inv() as usize]  {
            true
        } else { false }
    }
}

#[derive(Debug,Clone,Copy,Hash,EnumCount,EnumIter,FromRepr)]
enum Side {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}
impl Side{//fully done by copilot
    fn inv(&self) -> Side {
        match self {
            Side::Top => Side::Bottom,
            Side::Right => Side::Left,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
        }
    }
}

#[derive(Debug,Clone)]
struct MetaData{
    collapsed_into: Option<TileTypeIndex>,
    possibilities: Vec<TileTypeIndex>
}

impl Default for MetaData {//fully done by copilot(including this comment)
    fn default() -> Self {
        MetaData {
            collapsed_into: None,
            possibilities: (1..TileTypeIndex::COUNT).map(|i|
                TileTypeIndex::from_repr(i).unwrap()
            ).collect()
        }
    }
}

// fn compatiblity(possiblity: TileTypeIndex, pos: (usize,usize)) -> bool{
//     let left_tile = match i {
//         0 => true,
//         _ =>{
//             if let Some(tile_type) = grid[i-1][j].collapsed_into {
//                 grid[i][j].collapsed_into.unwrap().is_compatible_with(tile_type, Side::Right)
//             } else { true }
//             //if this has not collapsed, then obviously it's compatible
//         }
//     };
// }

fn reduce_grid(grid: &mut [[MetaData;DIM];DIM]) {//poorly optimised, can rework if speed is an issue
    const MAX_INDEX: usize= DIM-1;
    for j in 0..DIM {
        for i in 0..DIM {//iterate over 2d grid
            if grid[i][j].collapsed_into.is_none(){//is the element has not yet collapsed, perform reduction
                let mut impossible_indexes: Vec<usize> = Vec::new();
                for (index,possibility) in grid[i][j].possibilities.iter().enumerate() {
                    let left_tile = match i {
                        0 => true,
                        _ =>{
                            if let Some(tile_type) = grid[i-1][j].collapsed_into {
                                possibility.is_compatible_with(tile_type, Side::Left)
                            } else { true }
                            //if this has not collapsed, then obviously it's compatible
                        }
                    };

                    //this was 70% done by copilot(including this comment)
                    let right_tile = match i {
                        MAX_INDEX=> true,
                        _ =>{
                            if let Some(tile_type) = grid[i+1][j].collapsed_into {
                                possibility.is_compatible_with(tile_type, Side::Right)
                            } else { true }
                            //if this has not collapsed, then obviously it's compatible
                        }
                    };

                    //these next two were 100% done by copilot(including this comment)
                    let top_tile = match j  {
                        0 => true,
                        _ =>{
                            if let Some(tile_type) = grid[i][j-1].collapsed_into {
                                possibility.is_compatible_with(tile_type, Side::Top)
                            } else { true }
                            //if this has not collapsed, then obviously it's compatible
                        }
                    };

                    let bottom_tile = match j {
                        MAX_INDEX => true,
                        _ =>{
                            if let Some(tile_type) = grid[i][j+1].collapsed_into {
                                possibility.is_compatible_with(tile_type, Side::Bottom)
                            } else { true }
                            //if this has not collapsed, then obviously it's compatible
                        }
                    };
                    if !left_tile || !right_tile || !top_tile || !bottom_tile {
                        impossible_indexes.push(index);
                    }
                }
                for imp_index in impossible_indexes.iter().rev() {//this was suggested by chatgpt, i would have never thought of this
                    grid[i][j].possibilities.remove(*imp_index);
                }
            }
            if let Some(collapsed_type) = grid[i][j].collapsed_into{
                grid[i][j].possibilities = vec![collapsed_type];
            }
        }
    }
}

//noinspection ALL
#[macroquad::main("Title")]
async fn main() {
    let tiles : [Texture2D;12]  = [
        load_texture("sprites/none.png").await.unwrap(),
        load_texture("sprites/blank.png").await.unwrap(),
        load_texture("sprites/h_line.png").await.unwrap(),
        load_texture("sprites/v_line.png").await.unwrap(),
        load_texture("sprites/L-bl.png").await.unwrap(),
        load_texture("sprites/L-br.png").await.unwrap(),
        load_texture("sprites/L-tl.png").await.unwrap(),
        load_texture("sprites/L-tr.png").await.unwrap(),
        load_texture("sprites/T-top.png").await.unwrap(),
        load_texture("sprites/T-right.png").await.unwrap(),
        load_texture("sprites/T-bottom.png").await.unwrap(),
        load_texture("sprites/T-left.png").await.unwrap(),
    ];

    let mut grid : [[MetaData;DIM];DIM] = Default::default();
    // let texture: Texture2D = load_texture("sprites/none.png").await.unwrap();
    // let mut slider_num = 0.0;

    let stringlist:Vec<String> = (1..TileTypeIndex::COUNT).map(|i|
        format!("{:?}",TileTypeIndex::from_repr(i).unwrap())
    ).collect();
    let stringlist:Vec<&str> = stringlist.iter().map(|i| i.as_str()).collect();
    let stringlist:[&str;TileTypeIndex::COUNT-1] = stringlist.try_into().unwrap();

    // trial space
    //println!("{:?}", .compatablity());
    let mut text1:String ="<your selection>".to_string();
    let mut clicked_tile = (0_usize,0_usize);
    let mut choice: usize = 0;
    // end trial space

    loop {
        clear_background(WHITE);
        // trial space
        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_texture_ex(&texture, 0., 0., WHITE, DrawTextureParams {
        //     dest_size: Some(vec2(100., &slider_num*100.)),
        //     ..Default::default()
        // });

        // end trial space

        for i in 0..DIM{
            for j in 0..DIM {
                let tile_type = grid[i][j].collapsed_into.unwrap_or(TileTypeIndex::None);//line generted by copilot (i didn't even know unwrap_or was a thing)
                draw_texture_ex(&tiles[tile_type as usize], i as f32 * SIZE, j as f32 * SIZE, WHITE, DrawTextureParams {
                    dest_size: Some(vec2(SIZE, SIZE)),
                    ..Default::default()
                });
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let mouse_pos = (mouse_pos.0 as usize / SIZE as usize, mouse_pos.1 as usize / SIZE as usize);
            //till here it's all copilot
            if mouse_pos.0 < DIM && mouse_pos.1 < DIM {
                text1 = format!("{:?}" , mouse_pos).to_string();
                clicked_tile = mouse_pos;
            }

        }

        widgets::Window::new(hash!(),vec2(400.,200.), vec2(320.,400.))
            .ui(&mut *root_ui(), |ui| {
                    //ui.slider(hash!(), "slider", 0.00..5.00, &mut slider_num);
                    ui.label(None, text1.as_str());

                    ui.button(None, "collapse").then(||{
                        grid[clicked_tile.0][clicked_tile.1].collapsed_into = Some(TileTypeIndex::from_repr(choice as usize + 1).unwrap());
                    });
                    ui.button(None, "reduce").then(||{
                        reduce_grid(&mut grid);
                    });
                    ui.combo_box(hash!(), "Tile Type", &stringlist, &mut choice);
                    ui.label(None, format!("choice: {}", choice).as_str());
                    ui.label(None, "Possiblities:");
                    for i in grid[clicked_tile.0][clicked_tile.1].possibilities.iter() {
                        ui.label(None, format!("{:?}", i).as_str());
                    }
                    

            });

        next_frame().await
    }
}