mod cube;
mod cube_bfs;
mod drawing;
use std::collections::VecDeque;
// use std::collections::HashSet;
// use std::collections::HashMap;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use std::fs::File;
use std::io::Write;


use cube::Cube;
// use crate::cube_bfs;


fn gen_all_dr(){
    let c = Cube{
        corners: 70936234050,
        edges: 74381997087197250,
    };
    // let c = Cube{
    //         corners: 69828936738,
    //         edges: 74381997087264900,
    //     };
    let moves:Vec<u8> = vec![1, 2, 3, 4, 11, 12, 13, 14, 15, 16, 21, 22, 23, 24];

    let mut states: [FxHashSet<Cube>; 14] = Default::default();

    states[0].insert(c);
    let mut i = 0;
    // println!("{:?}",states);
    for idx in 0..states.len()-1{
        println!("Optimal: {}. States: {}",idx,states[idx].len());
        let mut next_states: FxHashSet<Cube> = FxHashSet::default();
        
        for state in &states[idx] {
            i+=1;
            if i%1_000_000 == 0{
                println!("{},{}",i, next_states.len())
            }
            for movee in &moves{
                let new_state = &state.perform_move(*movee);
                if idx > 0{
                    if !states[idx].contains(new_state) && !states[idx-1].contains(new_state){
                        next_states.insert(*new_state);
                    }
                }
                else {
                    if !states[idx].contains(new_state){
                        next_states.insert(*new_state);
                    }
                }
        }
        }
        states[idx + 1] = next_states;
    }

    // let mut q:VecDeque<(u64, u64, [u8;8],u8)> = VecDeque::new();
}

fn other_dr(){
    // let c = Cube{
    //     corners: 69828936738,
    //     edges: 74381997087264900,
    // };

    let c = Cube{
        corners: 70936234050,
        edges: 74381997087197250,
    };

    let mut set: FxHashSet<Cube> = FxHashSet::default();
    let moves:Vec<u8> = vec![1, 2, 3, 4, 11, 12, 13, 14, 15, 16, 21, 22, 23, 24];
    let mut q:VecDeque<Cube> = VecDeque::from(vec![c]);
    let mut i = 0;
    while let Some(nc) = q.pop_front(){
        i+=1;
        if i%1_000_000 == 0{
            println!("{},{}",i, set.len())
        }
        // if !set.contains(&nc){
            // set.insert(nc);
            for movee in &moves{
                let new_state = &nc.perform_move(*movee);
                if !set.contains(&new_state){
                    set.insert(*new_state);
                    q.push_back(*new_state);
                }
            }
        // }
    }
    println!("{}", set.len())
}

fn main(){
    // let o =  Cube{
    //     corners: 70936234050,
    //     edges: 74381997087197250,
    // };

    // println!("{:?}",o);

    // other_dr();
    let res = drawing::show_all_dr_by_ec(4,2);

    let mut file = File::create("full.html").expect("creating file");

    // Write data to the file
    file.write_all(&res.as_bytes());
    // Ok(())
    // let solutions = cube_bfs::gen_all_dr_solutions();
    // let k = solutions.values();
    // let mut p = 0;
    // for o in k{
    //     // if (o.dr_corner_count() >3) && (o.dr_edge_count() >1){
    //         let solution = o.get_solution_from_scr();
        
    //         let solution_string = cube_bfs::int_moves_to_str(solution);
    //         let mut new_cube = Cube::new();
    //         for movee in o.moves{
    //             new_cube = new_cube.perform_move(movee);
    //         }
    //         let svg = drawing::get_cube_svg(new_cube);
            
    //         println!("{} {:?} ",svg,solution_string);
    //         break;
        
    // }
    // println!("{p} ");
    // gen_all_dr();

    // let svg = drawing::example();
    // println!("{svg}");

    // let number: u64 = 42;
    // let binary_string = format!("{:b}", number);
    // println!("Binary representation: {}", binary_string);

    // let o = Cube{
    //     corners: 247132686368,
    //     edges: 407901468851537952,
    // };

    // let (o, trash) = o.do_scramble("F' B2 L B' U' B' R D2 F R2 L2 F B L2 D2 F' R2 U2 B' R' F2".to_string());

    // let mut a = drawing::get_cube_svg(o);
    // for i in 0..1_000_000{
    //     let c = drawing::get_cube_svg(o);
    //     a = c;
    // }
    // println!("{}",a);

    // let mut file = File::create("t2.svg").expect("creating file");

    // // Write data to the file
    // file.write_all(&a.as_bytes());
    // Ok(())

}