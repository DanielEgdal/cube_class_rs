mod base_cube;
mod cube_bfs;
use rayon::collections::binary_heap::Iter;
// use base_cube::BaseCube;
use rayon::prelude::*;
use rustc_hash::{FxHashMap,FxHashSet};
use crate::base_cube::*;
use crate::cube_bfs::*;
// use std::fs::File;
// use std::io::Write;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::prelude::IteratorRandom;

fn get_dist_subset<'a>(u_edges: &'a usize, u_corners: &'a usize, dist: &'a FxHashMap<DRToHTRCube,Cube_BFS>) -> Vec<(&'a DRToHTRCube,&'a Cube_BFS)> { // impl Iterator<Item = (&'a base_cube::DRToHTRCube, &'a cube_bfs::Cube_BFS)>
    let dist_filtered: Vec<_> = dist.iter()
        // .map(| (&cube, bfs) | (cube.count_non_htr_edges(),cube.count_non_htr_corners(), bfs.moves))
        .filter(|(cube,bfs)| cube.edges_minus() as usize == *u_edges && cube.corners_minus() as usize== *u_corners)
        // .map(|(cube,bfs)| (cube,bfs.moves))
        .collect();

    dist_filtered
}

fn get_random_case_of_len(len: usize, dist: &FxHashMap<DRToHTRCube,Cube_BFS>, htr_checker: &FxHashSet<DRToHTRCube>) -> Result<(String, String),String>{
    // let keys: Vec<_> = dist.keys().collect();
    // while let Some(chosen) =  keys.choose(&mut rand::thread_rng()){
    loop {
        if let Some((chosen, bfs_res)) = dist.iter().choose(&mut rand::thread_rng()){
            let solution = Cube_BFS::dr_to_htr_fixed_len(*chosen, &htr_checker, len);
            match solution{
                Ok(s) => return Ok((Cube_BFS::int_moves_to_str(bfs_res.moves.to_vec()), Cube_BFS::int_moves_to_str(s.to_vec()))),
                Err(e) => continue, 
            }
        }
        else{
            return Err("Somehow failed".to_string())
        }
    }
}

fn get_further_selected_case<'a>(len: usize, qt_len: usize, cases: &Vec<(&DRToHTRCube, &Cube_BFS)>, htr_checker: &FxHashSet<DRToHTRCube>) -> Result<(String, String),String>{
    loop {
        if let Some((chosen, bfs_res)) = cases.iter().choose(&mut rand::thread_rng()){
            let solution = Cube_BFS::dr_to_htr_max_len_fixed_qt(**chosen, &htr_checker, len, qt_len);
            match solution{
                Ok(s) => return Ok((Cube_BFS::int_moves_to_str(bfs_res.moves.to_vec()), Cube_BFS::int_moves_to_str(s.to_vec()))),
                Err(e) => continue, 
            }
        }
        else{
            return Err("Somehow failed".to_string())
        }
    }
}

fn htr_practice(){
    let bfs = Cube_BFS::dr_to_htr_dist();
    let htr_checker = cube_bfs::Cube_BFS::dr_to_htr_checker();
    let htr_minus: Vec<(&DRToHTRCube, &Cube_BFS)> = get_dist_subset(&4,&4,&bfs);
    println!("htrminus size {}",htr_minus.len());
    let mut start_vec = vec![];
    for i in 0..10{
        // start_vec.push(get_random_case_of_len(6,&bfs,&htr_checker).unwrap())
        start_vec.push(get_further_selected_case(7, 2,&htr_minus,&htr_checker).unwrap())
        
    }
    println!("{:?}",start_vec);
}


fn dr_get_dist_subset<'a>(u_edges: &'a usize, u_corners: &'a usize, dist: &'a FxHashMap<EOToDRCube,DR_Cube_BFS>) -> Vec<(&'a EOToDRCube,&'a DR_Cube_BFS)> { // impl Iterator<Item = (&'a base_cube::DRToHTRCube, &'a cube_bfs::Cube_BFS)>
    let dist_filtered: Vec<_> = dist.iter()
        // .map(| (&cube, bfs) | (cube.count_non_htr_edges(),cube.count_non_htr_corners(), bfs.moves))
        .filter(|(cube,bfs)| cube.edges_minus() as usize == *u_edges && cube.corners_minus() as usize== *u_corners)
        // .map(|(cube,bfs)| (cube,bfs.moves))
        .collect();

    dist_filtered
}

fn dr_get_random_case_of_len(len: usize, dist: &Vec<(&EOToDRCube, &DR_Cube_BFS)>) -> Result<(String),String>{
    // let keys: Vec<_> = dist.keys().collect();
    // while let Some(chosen) =  keys.choose(&mut rand::thread_rng()){
    loop {
        if let Some((chosen, bfs_res)) = dist.iter().choose(&mut rand::thread_rng()){
            // let solution = DR_Cube_BFS::solve_eo_to_dr(**chosen);
            let sol_len = bfs_res.depth;
            if sol_len != len{
                continue
            }
            return Ok((DR_Cube_BFS::int_moves_to_str(bfs_res.moves.to_vec())))
        }
        else{
            return Err("Somehow failed".to_string())
        }
    }
}

fn dr_practice(){
    let bfs = DR_Cube_BFS::eo_to_dr_dist();
    let dr_minus: Vec<(&EOToDRCube, &DR_Cube_BFS)> = dr_get_dist_subset(&8,&7,&bfs);
    println!("drminus size {}",dr_minus.len());
    let mut start_vec = vec![];
    for i in 0..10{
        // start_vec.push(get_random_case_of_len(6,&bfs,&htr_checker).unwrap())
        start_vec.push(dr_get_random_case_of_len(6, &dr_minus).unwrap())
        
    }
    println!("{:?}",start_vec);
}


fn main(){
    // // let nc = c.b();
    // let (mut scrambled_cube, scramble_vec) = c.do_scramble(scramble);


    // for i in 0u64..10_000_000{
    //     for movee in &scramble_vec{
    //         scrambled_cube = scrambled_cube.perform_move(*movee);
    //     }
        
    // }
    // let c = base_cube::DRToHTRCube::construct();

    // println!("old: {:?}",c);
    // let c = EOToDRCube::construct();
    // println!("{:?}",c);
    // let (scrambled_cube, scramble) = c.do_scramble("R U L".to_string());
    // // let (solution_arr, sol_len) = cube_bfs::Cube_BFS::solve_dr_to_htr(&scramble,&htr_checker).unwrap();
    // // println!("{:?}",scrambled_cube);
    // // println!("{:?}",scrambled_cube.count_non_htr_corners());
    // // std::process::exit(0);
    // // println!("{:?}",c);
    
    // // cube_bfs::Cube_BFS::dr_to_htr_checker();
    // // let (scrambled_cube, scramble) = c.do_scramble("U L2 U' L2 U2 B2 D L2 D'".to_string());
    // println!("{:?}",scrambled_cube);
    // println!("{:?},{:?}",scrambled_cube.edges_minus(),scrambled_cube.corners_minus());

    // dr_practice();

    let tmp = cube_bfs::bfs::<EOOnlyCube>();

    println!("{:?}",tmp)

    // 
    // let sol_string = cube_bfs::Cube_BFS::int_moves_to_str(solution_arr.to_vec());
    // println!("{sol_string}");

    // Result<[u8;20],&str>
    // let sols: Vec<Result<[u8;20],&str>> = bfs.par_iter().map(|(cube,_)| cube_bfs::Cube_BFS::dr_to_htr_fixed_len(*cube, &htr_checker, 4)).collect();
    // println!("{:?}",sols);
    
    
    // for (cube,_) in bfs{
    //     // let t = cube.moves.to_vec();
    //     let (sol, sol_len )= cube_bfs::Cube_BFS::solve_dr_to_htr(cube, &htr_checker).unwrap();
    //     println!("{:?}",sol);
    // }
    // println!("{:?}",solution_arr);
    
    // let scramble = "D' L2 U F2 R2 U2 R2 U F2 U F2".to_string();
    // let (mut scrambled_cube, scramble_vec) = c.do_scramble(scramble);

    // let cbf = cube_bfs::Cube_BFS::new();
    // let ov = cbf.dr_to_htr_dist();
    // // println!("{:?}",ov);
    // let sol = ov.get(&scrambled_cube).unwrap();
    // println!("{:?}",cube_bfs::Cube_BFS::int_moves_to_str(sol.invert_moves()));
    // println!("after move: {:?}",scrambled_cube);
    
}

