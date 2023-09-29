use std::collections::HashMap;

use crate::cube::Cube;

const FULL_IMAGE_SIZE: u32 = 450;
const SQUARE_SIZE: u32 = FULL_IMAGE_SIZE/12;

const FULL_TEMPLATE: &str = r#"
    <svg xmlns="http://www.w3.org/2000/svg" width="{FULL_IMAGE_SIZE}" height="{FULL_IMAGE_SIZE}" version="1.1">
        <rect x="0" y="0" width="{FULL_IMAGE_SIZE}" height="{FULL_IMAGE_SIZE}" fill="white"/>
        {layers}
    </svg>
"#;

// Define a HashMap outside of a function
const EDGE_MAP: [[&str; 2]; 12] = [
    ["yellow", "blue"],
    ["yellow", "orange"],
    ["yellow", "red"],
    ["yellow", "green"],
    ["blue", "orange"],
    ["blue", "red"],
    ["green", "red"],
    ["green", "orange"],
    ["white", "green"],
    ["white", "orange"],
    ["white", "red"],
    ["white", "blue"]];

const CORNER_MAP: [[&str; 3]; 8] = [
    ["yellow", "blue", "red"],
    ["yellow", "orange", "blue"],
    ["yellow", "red", "green"],
    ["yellow", "green", "orange"],
    ["white", "green", "red"],
    ["white", "orange", "green"],
    ["white", "red", "blue"],
    ["white", "blue", "orange"]];

pub fn get_colours_from_cube(cube: Cube) -> [[&'static str; 9]; 6]{
    let mut cube_colours = [[""; 9]; 6]; // White, Blue, Orange, Red, Green, Yellow
    let mut binary_edges = format!("{:b}", cube.edges);
    let padding = 60- binary_edges.len();
    binary_edges = "0".repeat(padding) + &binary_edges;
    let mut i_edges = 0;
    let mut u_edge_layer_count = 0;
    let mut e_edge_count = 0;
    let mut d_edge_layer_count = 3;
    while i_edges < 59{
        let eo_char = binary_edges.chars().nth(i_edges).expect("edge string within range");
        let eo = eo_char as usize - '0' as usize;
        let piece_str = &binary_edges[i_edges+1..i_edges+5];  
        let piece_id = u32::from_str_radix(piece_str, 2).expect("string only consists of binary");
        if i_edges < 19{ // Edges touching the U layer

            let mut temp_u_edge_count = u_edge_layer_count;
            if temp_u_edge_count == 1{ // Ugly fix because the UR UL piece are in different orders compared to the U layer
                temp_u_edge_count = 2;
            }
            else if temp_u_edge_count == 2{
                temp_u_edge_count = 1;
            }

            let u_idx = 1 + (2*temp_u_edge_count);

            cube_colours[0][u_idx] = EDGE_MAP[piece_id as usize][eo as usize];
            cube_colours[temp_u_edge_count+1][1] = EDGE_MAP[piece_id as usize][((eo+1)%2) as usize];
            u_edge_layer_count+=1;
        }
        else if i_edges < 39{ // Edges touching the E layer
            if e_edge_count == 0{
                cube_colours[4][3] = EDGE_MAP[piece_id as usize][eo as usize];
                cube_colours[2][5] = EDGE_MAP[piece_id as usize][((eo+1)%2) as usize];
            }
            else if e_edge_count == 1{
                cube_colours[4][5] = EDGE_MAP[piece_id as usize][eo as usize];
                cube_colours[3][3] = EDGE_MAP[piece_id as usize][((eo+1)%2) as usize];
            }

            else if e_edge_count == 2{
                cube_colours[1][3] = EDGE_MAP[piece_id as usize][eo as usize];
                cube_colours[3][5] = EDGE_MAP[piece_id as usize][((eo+1)%2) as usize];
            }

            else{
                cube_colours[1][5] = EDGE_MAP[piece_id as usize][eo as usize];
                cube_colours[2][3] = EDGE_MAP[piece_id as usize][((eo+1)%2) as usize];
            }
            e_edge_count+=1;
        }
        else{ // D layer edges
            let u_idx = 1 + (2*d_edge_layer_count);
            let mut temp_u_idx = u_idx;
            if temp_u_idx == 1{ // Ugly fix because the DF and DB piece are in different orders compared to the U layer
                temp_u_idx = 7;
            }
            else if temp_u_idx == 7{
                temp_u_idx = 1;
            }
            cube_colours[5][temp_u_idx] = EDGE_MAP[piece_id as usize][eo as usize];
            cube_colours[d_edge_layer_count+1][7] = EDGE_MAP[piece_id as usize][((eo+1)%2) as usize];
            if d_edge_layer_count > 0{
                d_edge_layer_count-=1;
            }
        }
        i_edges+=5;
    }
    // Corners
    let mut binary_corners = format!("{:b}", cube.corners);
    let padding_corners = 40- binary_corners.len();
    binary_corners = "0".repeat(padding_corners) + &binary_corners;
    let mut i_corners = 0;
    while i_corners < 39{
        let co_str = &binary_corners[i_corners..i_corners+2];  
        let co_id = u32::from_str_radix(co_str, 2).expect("string only consists of binary");

        let piece_str = &binary_corners[i_corners+2..i_corners+5];  
        let piece_id = u32::from_str_radix(piece_str, 2).expect("string only consists of binary");

        let (c1,c2,c3) = iterate_over_corner_map(piece_id,co_id);
        if i_corners == 0{
            cube_colours[0][0] = c1;
            cube_colours[1][2] = c2;
            cube_colours[2][0] = c3;
        }
        else if i_corners == 5{
            cube_colours[0][2] = c1;
            cube_colours[3][2] = c2;
            cube_colours[1][0] = c3;
        }
        else if i_corners == 10{
            cube_colours[0][6] = c1;
            cube_colours[2][2] = c2;
            cube_colours[4][0] = c3;
        }
        else if i_corners == 15{
            cube_colours[0][8] = c1;
            cube_colours[4][2] = c2;
            cube_colours[3][0] = c3;
        }
        else if i_corners == 20{
            cube_colours[5][0] = c1;
            cube_colours[4][6] = c2;
            cube_colours[2][8] = c3;
        }
        else if i_corners == 25{
            cube_colours[5][2] = c1;
            cube_colours[3][6] = c2;
            cube_colours[4][8] = c3;
        }
        else if i_corners == 30{
            cube_colours[5][6] = c1;
            cube_colours[2][6] = c2;
            cube_colours[1][8] = c3;
        }
        else if i_corners == 35{
            cube_colours[5][8] = c1;
            cube_colours[1][6] = c2;
            cube_colours[3][8] = c3;
        }
        i_corners+=5;
    }
    cube_colours[0][4] = "white";
    cube_colours[1][4] = "blue";
    cube_colours[2][4] = "orange";
    cube_colours[3][4] = "red";
    cube_colours[4][4] = "green";
    cube_colours[5][4] = "yellow";
    cube_colours

}

fn iterate_over_corner_map(piece_id:u32, co_id:u32) -> (&'static str,&'static str,&'static str){
    let c1 = CORNER_MAP[piece_id as usize][((co_id)%3) as usize];
    let c2 = CORNER_MAP[piece_id as usize][((co_id+1)%3) as usize];
    let c3 = CORNER_MAP[piece_id as usize][((co_id+2)%3) as usize];
    (c1,c2,c3)
    
}

fn make_layer(x_start: u32, y_start: u32, colours: [&str;9]) -> String {
    // println!("{:?}",colours);
    let mut layer = String::new();

    for i in 0..3 {
        for j in 0..3 {
            let c = colours[((i * 3) + j) as usize];
            let x = x_start + (SQUARE_SIZE * j);
            let y = y_start + (SQUARE_SIZE * i);
            // println!("{c} {x} {y} ");
            let sticker = format!(
                r#"<rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{c}" stroke="black" stroke-width="2"/>"#,
                x = x,
                y = y,
                w = SQUARE_SIZE,
                h = SQUARE_SIZE,
                c = c
            );
            layer += &sticker;
        }
    }
    layer
}

pub fn get_cube_svg(cube:Cube) -> String{
    let mut svg_layers = String::new();
    let coloured_layers = get_colours_from_cube(cube);
    svg_layers += &make_layer(3*SQUARE_SIZE,0,coloured_layers[0]);
    svg_layers += &make_layer(9*SQUARE_SIZE,3*SQUARE_SIZE,coloured_layers[1]);
    svg_layers += &make_layer(0,3*SQUARE_SIZE,coloured_layers[2]);
    svg_layers += &make_layer(6*SQUARE_SIZE,3*SQUARE_SIZE,coloured_layers[3]);
    svg_layers += &make_layer(3*SQUARE_SIZE,3*SQUARE_SIZE,coloured_layers[4]);
    svg_layers += &make_layer(3*SQUARE_SIZE,6*SQUARE_SIZE,coloured_layers[5]);
    let result = FULL_TEMPLATE.replace("{layers}", &svg_layers).replace("{FULL_IMAGE_SIZE}", &FULL_IMAGE_SIZE.to_string());
    result
}

// pub fn example() -> String{
//     let co: Vec<&str> = vec!["white"; 9];
//     let layers = make_layer(0, 0, co);

//     let result = FULL_TEMPLATE.replace("{layers}", &layers).replace("{FULL_IMAGE_SIZE}", &FULL_IMAGE_SIZE.to_string());
//     // println!("{}", result);
//     result
// }