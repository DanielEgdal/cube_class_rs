mod cube;
use std::collections::VecDeque;
// use std::collections::HashSet;
// use std::collections::HashMap;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

use cube::Cube;


fn gen_all_dr(){
    // let c = Cube{
    //     corners: 70936234050,
    //     edges: 74381997087197250,
    // };
    let c = Cube{
            corners: 69828936738,
            edges: 74381997087264900,
        };
    let moves:Vec<u8> = vec![1, 2, 3, 4, 11, 12, 13, 14, 15, 16, 21, 22, 23, 24];

    let mut states: [FxHashSet<Cube>; 14] = Default::default();

    states[0].insert(c);
    let mut i = 0;
    // println!("{:?}",states);
    for idx in 0..states.len()-1{
        println!("Optimal: {}. States: {}",idx,states[idx].len());
        let mut next_states: FxHashSet<Cube> = FxHashSet::default();
        i+=1;
        if i%1_000_000 == 0{
            println!("{},{}",i, next_states.len())
        }
        for state in &states[idx] {
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
    let c = Cube{
        corners: 69828936738,
        edges: 74381997087264900,
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
                    set.insert(nc);
                    q.push_back(*new_state);
                }
            }
        // }
    }
    println!("{}", set.len())
}

fn main(){
    let o =  Cube{
        corners: 70936234050,
        edges: 74381997087197250,
    };
    println!("{:?}",o);

    other_dr();
}