use std::collections::VecDeque;
use std::hash::Hash;
// use std::collections::HashSet;
// use std::collections::HashMap;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use crate::base_cube;
// mod base_cube;
// use base_cube::BaseCube;
use crate::base_cube::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)] 
pub struct cube_bfs_helper<T: BaseCube>{
    pub cube: T,
    pub depth: usize,
    pub moves: [u8;20],
}

impl<T: BaseCube> cube_bfs_helper <T> {
    fn apply_move(mut self, movee:u8) -> Self{
        self.cube = self.cube.perform_move(movee);
        self.moves[self.depth] = movee;
        self.depth+=1;
        self
    }

    fn new() -> Self{
        Self { cube: T::new(), depth: 0, moves: [0;20] }
    }
}

pub fn bfs<T: BaseCube + Eq + PartialEq + Hash + Clone + Copy >() -> [usize;20]{

    let start_cube = cube_bfs_helper::<T>::new();
    let mut solutions: FxHashMap<T,cube_bfs_helper<T>> = FxHashMap::default();
    let moves:Vec<u8> = vec![1, 2, 3, 4, 5, 6, 11, 12, 13, 14, 15, 16, 21, 22, 23, 24, 25, 26];
    let mut q:VecDeque<cube_bfs_helper<T>> = VecDeque::from(vec![start_cube]);
    let mut overview = [0;20];
    // let mut htr_minus = [0;]
    // let mut diffent_cases: FxHashSet<(u32,u32)> = FxHashSet::default();

    let mut i = 0;
    while let Some(nc) = q.pop_front(){
        i+=1;
        if i%1_000_000 == 0{
            println!("{},{}",i, solutions.len())
        }
        if !solutions.contains_key(&nc.cube){ // The very intial state as 0
            overview[nc.depth as usize] +=1;
            solutions.insert(nc.cube,nc.clone());
        }

        for movee in &moves{
            let new_state: &mut cube_bfs_helper<T> = &mut nc.apply_move(*movee);
            if !solutions.contains_key(&new_state.cube){
                overview[new_state.depth as usize] +=1;
                solutions.insert(new_state.cube,new_state.clone());
                q.push_back(*new_state);
            }
        }
        assert_eq!(solutions.len(),overview.iter().sum());
    println!("len solutions {:?}",solutions.len())
    }

    return overview
}

fn int_moves_to_str(moves:Vec<u8>) -> String{
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

fn invert_moves(moves:[u8;20])-> Vec<u8>{
    let mut reverse_sol = Vec::new();
    for movee in moves.iter().rev(){
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


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)] 
pub struct Cube_BFS {
    pub cube: DRToHTRCube,
    pub moves: [u8;20], // Probably 11 when looking at dr
    pub depth: usize,
    pub qt: usize,
}

impl Cube_BFS{
    pub fn new() -> Self{
        Self {
            cube: DRToHTRCube::new(),
            moves: [0;20],
            depth: 0,
            qt: 0
        }
    }

    pub fn apply_move(mut self, movee:u8) -> Self{
        self.cube = self.cube.perform_move(movee);
        self.moves[self.depth] = movee;
        self.depth+=1;
        if (movee < 10 || movee > 20){
            self.qt +=1;
        }
        self
    }

    pub fn invert_moves(self)-> Vec<u8>{
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

    pub fn dr_to_htr_dist() -> FxHashMap<DRToHTRCube,Cube_BFS> {
        let start_cube = Cube_BFS::new();
        let mut solutions: FxHashMap<DRToHTRCube,Cube_BFS> = FxHashMap::default();
        let moves:Vec<u8> = vec![3, 4, 11, 12, 13, 14, 15, 16, 23, 24];
        let mut q:VecDeque<Cube_BFS> = VecDeque::from(vec![start_cube]);
        let mut overview: [i32; 20] = [0;20];
        // let mut htr_minus = [0;]
        // let mut diffent_cases: FxHashSet<(u32,u32)> = FxHashSet::default();

        let mut i = 0;
        while let Some(nc) = q.pop_front(){
            i+=1;
            if i%1_000_000 == 0{
                println!("{},{}",i, solutions.len())
            }
            for movee in &moves{
                let new_state = &mut nc.apply_move(*movee);
                if !solutions.contains_key(&new_state.cube){
                    // diffent_cases.insert((new_state.cube.count_non_htr_corners(),new_state.cube.count_non_htr_edges()));
                    // htr_minus[&(new_state.cube.count_non_htr_corners(),new_state.cube.count_non_htr_edges())] += 1;
                    overview[new_state.depth as usize] +=1;
                    solutions.insert(new_state.cube,new_state.clone());
                    q.push_back(*new_state);
                }
            }
        }
        // let mut counts: FxHashMap<(u32,u32), u32> = FxHashMap::default();
        // println!("{:?}",overview);
        println!("{:?}",overview);
        println!("{}",solutions.len());
        // println!("{}",solutions.len());
        // for (mut cube, _) in &solutions{
        //     let entry = counts.entry((cube.count_non_htr_corners(),cube.count_non_htr_edges())).or_insert(0);
        //     *entry += 1;
        // }
        // println!("{:?}",counts);
        solutions
        // diffent_cases
    }

    pub fn dr_to_htr_checker() -> FxHashSet<DRToHTRCube>{
        let start_cube = Cube_BFS::new();
        let mut cases: FxHashSet<DRToHTRCube> = FxHashSet::default();
        let moves:Vec<u8> = vec![11, 12, 13, 14, 15, 16];
        let mut q:VecDeque<Cube_BFS> = VecDeque::from(vec![start_cube]);

        let mut i = 0;
        while let Some(nc) = q.pop_front(){
            i+=1;
            if i%1_000_000 == 0{
                println!("{},{}",i, cases.len())
            }
            for movee in &moves{
                let new_state = &mut nc.apply_move(*movee);
                if !cases.contains(&new_state.cube){
                    // diffent_cases.insert((new_state.cube.count_non_htr_corners(),new_state.cube.count_non_htr_edges()));
                    // htr_minus[&(new_state.cube.count_non_htr_corners(),new_state.cube.count_non_htr_edges())] += 1;
                    cases.insert(new_state.cube);
                    q.push_back(*new_state);
                }
            }
        }
        println!("{:?}",cases.len());

        cases
    }

    pub fn dr_to_htr_fixed_len<'a>(scrambled_cube: DRToHTRCube, htr_checker: &'a FxHashSet<DRToHTRCube>, sol_len: usize) -> Result<[u8;20],&'a str>{
        let mut cube = DRToHTRCube::new();
        // for movee in scramble{
        //     cube = cube.perform_move(*movee)
        // }
        let mut bfs = Cube_BFS::new();
        bfs.cube = scrambled_cube;

        // let htr_checker = Self::dr_to_htr_checker();
        let mut seen: FxHashSet<DRToHTRCube> = FxHashSet::default();

        let moves:Vec<u8> = vec![3, 4, 11, 12, 13, 14, 15, 16, 23, 24];
        let mut q:VecDeque<Cube_BFS> = VecDeque::from(vec![bfs]);

        while let Some(nc) = q.pop_front(){
            if nc.depth > sol_len{
                break;
            }
            if !seen.contains(&nc.cube){
                seen.insert(nc.cube);
                for movee in &moves{
                    let new_state = &mut nc.apply_move(*movee);
                    if htr_checker.contains(&new_state.cube){
                        if new_state.depth == sol_len{
                            return Ok(new_state.moves)
                        }
                        else{
                            return Err("doesnt fit sol len")
                        }
                    }
                    q.push_back(*new_state);
                }
            }
            
        }
        Err("No solution found")

    }

    pub fn dr_to_htr_max_len_fixed_qt<'a>(scrambled_cube: DRToHTRCube, htr_checker: &'a FxHashSet<DRToHTRCube>, max_len: usize, qt_len: usize) -> Result<[u8;20],&'a str>{
        let mut cube = DRToHTRCube::new();
        // for movee in scramble{
        //     cube = cube.perform_move(*movee)
        // }
        let mut bfs = Cube_BFS::new();
        bfs.cube = scrambled_cube;

        // let htr_checker = Self::dr_to_htr_checker();
        let mut seen: FxHashSet<DRToHTRCube> = FxHashSet::default();

        let moves:Vec<u8> = vec![3, 4, 11, 12, 13, 14, 15, 16, 23, 24];
        let mut q:VecDeque<Cube_BFS> = VecDeque::from(vec![bfs]);

        while let Some(nc) = q.pop_front(){
            if nc.depth > max_len{
                break;
            }
            if !seen.contains(&nc.cube){
                seen.insert(nc.cube);
                for movee in &moves{
                    let new_state = &mut nc.apply_move(*movee);
                    if htr_checker.contains(&new_state.cube){
                        if new_state.depth <= max_len && new_state.qt == qt_len{
                            return Ok(new_state.moves)
                        }
                        else{
                            return Err("doesnt fit sol len")
                        }
                    }
                    q.push_back(*new_state);
                }
            }
            
        }
        Err("No solution found")

    }


    pub fn solve_dr_to_htr<'a>(scrambled_cube: DRToHTRCube, htr_checker: &'a FxHashSet<DRToHTRCube>) -> Result<([u8;20],usize),&'a str>{
        let mut cube = DRToHTRCube::new();
        // for movee in scramble{
        //     cube = cube.perform_move(*movee)
        // }
        let mut bfs = Cube_BFS::new();
        bfs.cube = scrambled_cube;

        // let htr_checker = Self::dr_to_htr_checker();
        let mut seen: FxHashSet<DRToHTRCube> = FxHashSet::default();

        let moves:Vec<u8> = vec![3, 4, 11, 12, 13, 14, 15, 16, 23, 24];
        let mut q:VecDeque<Cube_BFS> = VecDeque::from(vec![bfs]);

        while let Some(nc) = q.pop_front(){
            if !seen.contains(&nc.cube){
                seen.insert(nc.cube);
                for movee in &moves{
                    let new_state = &mut nc.apply_move(*movee);
                    if htr_checker.contains(&new_state.cube){
                        println!("done");
                        return Ok((new_state.moves,new_state.depth))
                    }
                    q.push_back(*new_state);
                }
            }
            
        }
        Err("No solution found")

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
}


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)] 
pub struct DR_Cube_BFS {
    pub cube: EOToDRCube,
    pub moves: [u8;20], // Probably 11 when looking at dr
    pub depth: usize,
    pub qt: usize,
}

impl DR_Cube_BFS{
    pub fn new() -> Self{
        Self {
            cube: EOToDRCube::new(),
            moves: [0;20],
            depth: 0,
            qt: 0
        }
    }

    pub fn apply_move(mut self, movee:u8) -> Self{
        self.cube = self.cube.perform_move(movee);
        self.moves[self.depth] = movee;
        self.depth+=1;
        if (movee < 10 || movee > 20){
            self.qt +=1;
        }
        self
    }

    pub fn invert_moves(self)-> Vec<u8>{
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

    pub fn eo_to_dr_dist() -> FxHashMap<EOToDRCube,DR_Cube_BFS> {
        let start_cube = DR_Cube_BFS::new();
        let mut solutions: FxHashMap<EOToDRCube,DR_Cube_BFS> = FxHashMap::default();
        let moves:Vec<u8> = vec![1, 2, 3, 4, 11, 12, 13, 14, 15, 16, 21, 22, 23, 24];
        let mut q:VecDeque<DR_Cube_BFS> = VecDeque::from(vec![start_cube]);
        let mut overview = [0;20];
        // let mut htr_minus = [0;]
        // let mut diffent_cases: FxHashSet<(u32,u32)> = FxHashSet::default();

        let mut i = 0;
        while let Some(nc) = q.pop_front(){
            i+=1;
            if i%1_000_000 == 0{
                println!("{},{}",i, solutions.len())
            }
            for movee in &moves{
                let new_state = &mut nc.apply_move(*movee);
                if !solutions.contains_key(&new_state.cube){
                    // diffent_cases.insert((new_state.cube.count_non_htr_corners(),new_state.cube.count_non_htr_edges()));
                    // htr_minus[&(new_state.cube.count_non_htr_corners(),new_state.cube.count_non_htr_edges())] += 1;
                    overview[new_state.depth as usize] +=1;
                    solutions.insert(new_state.cube,new_state.clone());
                    q.push_back(*new_state);
                }
            }
        }
        // let mut counts: FxHashMap<(u32,u32), u32> = FxHashMap::default();
        // println!("{:?}",overview);
        println!("{:?}",overview);
        println!("{}",solutions.len());
        // println!("{}",solutions.len());
        // for (mut cube, _) in &solutions{
        //     let entry = counts.entry((cube.count_non_htr_corners(),cube.count_non_htr_edges())).or_insert(0);
        //     *entry += 1;
        // }
        // println!("{:?}",counts);
        solutions
        // diffent_cases
    }

    pub fn solve_eo_to_dr<'a>(scrambled_cube: EOToDRCube) -> Result<([u8;20],usize),&'a str>{
        let cube = EOToDRCube::new();
        // for movee in scramble{
        //     cube = cube.perform_move(*movee)
        // }
        let mut bfs = DR_Cube_BFS::new();
        bfs.cube = scrambled_cube;

        // let htr_checker = Self::dr_to_htr_checker();
        let mut seen: FxHashSet<EOToDRCube> = FxHashSet::default();

        let moves:Vec<u8> = vec![1, 2, 3, 4, 11, 12, 13, 14, 15, 16, 21, 22, 23, 24];
        let mut q:VecDeque<DR_Cube_BFS> = VecDeque::from(vec![bfs]);

        while let Some(nc) = q.pop_front(){
            if !seen.contains(&nc.cube){
                seen.insert(nc.cube);
                for movee in &moves{
                    let new_state = &mut nc.apply_move(*movee);
                    if new_state.cube == cube {
                        return Ok((new_state.moves,new_state.depth))
                    }
                    q.push_back(*new_state);
                }
            }
            
        }
        Err("No solution found")

    }
}