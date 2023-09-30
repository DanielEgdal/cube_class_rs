use std::collections::VecDeque;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

use crate::cube::Cube;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)] 
pub struct Cube_BFS {
    pub cube: Cube,
    pub moves: [u8;11],
    pub depth: usize,
}

impl Cube_BFS{
    pub fn apply_move(mut self, movee:u8) -> Self{
        self.cube = self.cube.perform_move(movee);
        self.moves[self.depth] = movee;
        self.depth+=1;
        self
    }

    pub fn get_solution_from_scr(self)-> Vec<u8>{
        let mut reverse_sol = Vec::new();
        for movee in self.moves.iter().rev(){
            if *movee >0{
                if *movee < 10{
                    reverse_sol.push(*movee + 20);
                }
                else if *movee > 20{
                    reverse_sol.push(*movee - 20);
                }
                else{
                    reverse_sol.push(*movee);
                }
            }
        }
        reverse_sol
    }

    pub fn dr_edge_count(self)-> u8{ // TODO move this other class as this depends on the cube input being a DR cube.
        let mut edges_in_e: u8 = 0;
        let mut e_edges = self.cube.edges >> 20;
        for _ in 0..4{
            edges_in_e += (e_edges & 1) as u8;
            e_edges >>= 5;
        }
        edges_in_e   
    }

    pub fn dr_corner_count(self)-> u8{ //TODO as above
        let mut oriented_corners: u8 = 0;
        let mut temp_corners = self.cube.corners;
        for _ in 0..8{
            oriented_corners += ((temp_corners & 24) == 0) as u8;
            temp_corners >>= 5;
        }
        oriented_corners
    }
}

pub fn gen_all_dr_solutions()->FxHashMap<Cube,Cube_BFS>{
    let c = Cube{
        corners: 70936234050,
        edges: 74381997087197250,
    };

    let start_cube = Cube_BFS{
        cube: c,
        moves: [0;11],
        depth: 0,
    };

    let mut solutions: FxHashMap<Cube,Cube_BFS> = FxHashMap::default();
    let moves:Vec<u8> = vec![1, 2, 3, 4, 11, 12, 13, 14, 15, 16, 21, 22, 23, 24];
    let mut q:VecDeque<Cube_BFS> = VecDeque::from(vec![start_cube]);

    let mut i = 0;
    while let Some(nc) = q.pop_front(){
        i+=1;
        if i%1_000_000 == 0{
            println!("{},{}",i, solutions.len())
        }
        for movee in &moves{
            let new_state = &nc.apply_move(*movee);
            if !solutions.contains_key(&new_state.cube){
                solutions.insert(new_state.cube,new_state.clone());
                q.push_back(*new_state);
            }
        }
    }
    println!("{}",solutions.len());

    solutions

}

pub fn int_moves_to_str(moves:Vec<u8>) -> String{
    let mut str_solution = String::new();
    for imove in &moves{
        str_solution.push_str(match imove {
            1 => "R ", 
            2 => "L ", 
            3 => "U ", 
            4 => "D ", 
            5 => "F ", 
            6 => "B ", 
            11 => "R2 ", 
            12 => "L2 ", 
            13 => "U2 ", 
            14 => "D2 ", 
            15 => "F2 ", 
            16 => "B2 ", 
            21 => "R' ", 
            22 => "L' ", 
            23 => "U' ", 
            24 => "D' ", 
            25 => "F' ", 
            26 => "B' ", 
            0 => "",
                _ => unreachable!()
            })
    }
    str_solution
}