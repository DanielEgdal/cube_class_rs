use std::ops::{BitAnd, Shl, Shr, Add, Sub, BitOr, BitXor};
use std::fmt::{Display,Debug};
use std::collections::HashSet;
use std::iter::FromIterator;
use const_for::*;

const fn array_contains(array: &[usize], to_check: i32) -> bool{
    const_for!(i in 0..array.len() => {
        if array[i] == to_check as usize{
            return true
        }
    });
    return false
    
}

const fn blank_pieces_corners(pieces_to_blacken: &[usize], csize_f: usize) -> u64{
    let mut val = 0;
    const_for!(i in (0..8).rev() => {
        val = val << csize_f;
        // if !pieces_to_blacken.contains(&i) {
        if !array_contains(pieces_to_blacken,i){
            val += (2<<(csize_f-1))-1;
        }
    });
    val
}

const fn blank_pieces_edges(pieces_to_blacken: &[usize], esize_f: usize) -> u64{
    let mut val = 0;
    const_for!(i in (0i32..12).rev() => {
        val = val << esize_f;
        // if !pieces_to_blacken.contains(&i) {
        if !array_contains(pieces_to_blacken,i){
            val += (2<<(esize_f-1))-1;
        }
    });
    val
}

pub trait BaseCube: Sized {
    const DBR: usize = 0;
    const DBL: usize = 1;
    const DFR: usize = 2;
    const DFL: usize = 3;
    const UFR: usize = 4;
    const UFL: usize = 5;
    const UBR: usize = 6;
    const UBL: usize = 7;

    const DB: usize = 0;
    const DL: usize = 1;
    const DR: usize = 2;
    const DF: usize = 3;
    const BL: usize = 4;
    const BR: usize = 5;
    const FR: usize = 6;
    const FL: usize = 7;
    const UF: usize = 8;
    const UL: usize = 9;
    const UR: usize = 10;
    const UB: usize = 11;

    const ESIZE: usize;
    const CSIZE: usize;
    const CORNER_ORINENTATION: bool;
    const EDGE_ORINENTATION: bool;
    const CSIZE_F: usize = match Self::CORNER_ORINENTATION {
        true => Self::CSIZE + 2,
        false => Self::CSIZE,
    };
    const ESIZE_F: usize = match Self::EDGE_ORINENTATION {
        true => Self::ESIZE + 1,
        false => Self::ESIZE,
    };
    
    type Edge: TryFrom<u64> + BitXor<Output = Self::Edge> + BitOr<Output = Self::Edge> + BitAnd<Output = Self::Edge> + Shl<usize, Output = Self::Edge> + Shr<usize, Output = Self::Edge> + Add<Output = Self::Edge> + Sub<Output = Self::Edge> + Copy + Display;
    type Corner: TryFrom<u64> + BitXor<Output = Self::Corner> + BitOr<Output = Self::Corner> + BitAnd<Output = Self::Corner> + Shl<usize, Output = Self::Corner> + Shr<usize, Output = Self::Corner> + Add<Output = Self::Corner> + Sub<Output = Self::Corner> + Copy + Display;
    
    const RC_MASK: u64 = blank_pieces_corners(&[Self::DBR,Self::DFR,Self::UFR,Self::UBR],Self::CSIZE_F);
    const RE_MASK: u64 = blank_pieces_edges(&[Self::DR,Self::FR,Self::UR,Self::BR],Self::ESIZE_F);

    const LC_MASK: u64 = blank_pieces_corners(&[Self::UFL,Self::DFL,Self::UBL,Self::DBL],Self::CSIZE_F);
    const LE_MASK: u64 = blank_pieces_edges(&[Self::DL,Self::BL,Self::FL,Self::UL],Self::ESIZE_F);

    const DC_MASK: u64 = blank_pieces_corners(&[Self::DFL,Self::DBL,Self::DFR,Self::DBR],Self::CSIZE_F);
    const DE_MASK: u64 = blank_pieces_edges(&[Self::DR,Self::DL,Self::DF,Self::DB],Self::ESIZE_F);

    const UC_MASK: u64 = blank_pieces_corners(&[Self::UFR,Self::UFL,Self::UBR,Self::UBL],Self::CSIZE_F);
    const UE_MASK: u64 = blank_pieces_edges(&[Self::UR,Self::UL,Self::UF,Self::UB],Self::ESIZE_F);

    const FC_MASK: u64 = blank_pieces_corners(&[Self::UFR,Self::DFR,Self::DFL,Self::UFL],Self::CSIZE_F);
    const FE_MASK: u64 = blank_pieces_edges(&[Self::DF,Self::UF,Self::FL,Self::FR],Self::ESIZE_F);

    const BC_MASK: u64 = blank_pieces_corners(&[Self::DBR,Self::DBL,Self::UBL,Self::UBR],Self::CSIZE_F);
    const BE_MASK: u64 = blank_pieces_edges(&[Self::BR,Self::BL,Self::DB,Self::UB],Self::ESIZE_F);

    fn edge(&mut self) -> &mut Self::Edge;

    fn corner(&mut self) -> &mut Self::Corner;

    fn new() -> Self;

    fn twist_corner(corner: Self::Corner) -> Self::Corner{
        let u = corner & (Self::Corner::try_from(3).ok().unwrap()  << Self::CSIZE);
        let o = (u+(Self::Corner::try_from(1).ok().unwrap() << Self::CSIZE)+((u&(Self::Corner::try_from(1).ok().unwrap() << (Self::CSIZE+1)))>> 1))&(Self::Corner::try_from(3).ok().unwrap() << Self::CSIZE);
        o + (corner&((Self::Corner::try_from(2).ok().unwrap() <<(Self::CSIZE-1))-Self::Corner::try_from(1).ok().unwrap() ))
    }

    fn twist_corner_c(corner: Self::Corner) -> Self::Corner{
        let u = corner & (Self::Corner::try_from(3).ok().unwrap() << Self::CSIZE);
        let t = u + (Self::Corner::try_from(3).ok().unwrap() << Self::CSIZE);
        let o = (t - ((t & (Self::Corner::try_from(1).ok().unwrap() << (Self::CSIZE+1)))>>1)) & (Self::Corner::try_from(3).ok().unwrap() << Self::CSIZE);
        o + (corner&((Self::Corner::try_from(2).ok().unwrap()<<(Self::CSIZE-1))-Self::Corner::try_from(1).ok().unwrap()))
    }

    #[inline(always)]
    fn base_move_c(&mut self, p1: usize, p2: usize, p3: usize, p4: usize, mask: Self::Corner, twist: bool) -> Self::Corner{
        let blank_c = (Self::Corner::try_from(2).ok().unwrap()<< (Self::CSIZE_F-1) )-Self::Corner::try_from(1).ok().unwrap();
        let mut corners = *self.corner();
        let mut block1 = (corners >> (p1 * Self::CSIZE_F)) & blank_c;
        let mut block2 = (corners >> (p2 * Self::CSIZE_F)) & blank_c;
        let mut block3 = (corners >> (p3 * Self::CSIZE_F)) & blank_c;
        let mut block4 = (corners >> (p4 * Self::CSIZE_F)) & blank_c;
        if Self::CORNER_ORINENTATION && twist{
            block1 = Self::twist_corner_c(block1);
            block2 = Self::twist_corner(block2);
            block3 = Self::twist_corner_c(block3);
            block4 = Self::twist_corner(block4);
        }

        corners = corners & mask;
        corners = corners ^ ((block1 << (p2 * Self::CSIZE_F)) | (block2 << (p3 * Self::CSIZE_F)) |(block3 << (p4 * Self::CSIZE_F)) |(block4 << (p1 * Self::CSIZE_F)));
        corners
    }
    
    #[inline(always)]
    fn base_move_e(&mut self, p1: usize, p2: usize, p3: usize, p4: usize, mask: Self::Edge, flip: bool) -> Self::Edge{
        let blank_e = (Self::Edge::try_from(2).ok().unwrap()<< (Self::ESIZE_F-1) )-Self::Edge::try_from(1).ok().unwrap();
        let mut edges = *self.edge();
        let mut block1 = (edges >> (p1 * Self::ESIZE_F)) & blank_e;
        let mut block2 = (edges >> (p2 * Self::ESIZE_F)) & blank_e;
        let mut block3 = (edges >> (p3 * Self::ESIZE_F)) & blank_e;
        let mut block4 = (edges >> (p4 * Self::ESIZE_F)) & blank_e;

        if Self::EDGE_ORINENTATION && flip{
            let first_bit_flip = Self::Edge::try_from(2).ok().unwrap()<< (Self::ESIZE-1);
            block1 = block1 ^ first_bit_flip;
            block2 = block2 ^ first_bit_flip;
            block3 = block3 ^ first_bit_flip;
            block4 = block4 ^ first_bit_flip;
        }

        edges = edges & mask; 
        edges = edges ^ ((block1 << (p2 * Self::ESIZE_F)) | (block2 << (p3 * Self::ESIZE_F)) |(block3 << (p4 * Self::ESIZE_F)) |(block4 << (p1 * Self::ESIZE_F)));
        edges
    }

    #[inline(always)]
    fn base_double_e(&mut self, p1: usize, p2: usize, p3: usize, p4: usize, mask: Self::Edge) -> Self::Edge{
        let blank_e = (Self::Edge::try_from(2).ok().unwrap()<< (Self::ESIZE_F-1) )-Self::Edge::try_from(1).ok().unwrap();
        let mut edges = *self.edge();
        let block1 = (edges >> (p1 * Self::ESIZE_F)) & blank_e;
        let block2 = (edges >> (p2 * Self::ESIZE_F)) & blank_e;
        let block3 = (edges >> (p3 * Self::ESIZE_F)) & blank_e;
        let block4 = (edges >> (p4 * Self::ESIZE_F)) & blank_e;

        edges = edges & mask;
        edges = edges ^ ((block1 << (p2 * Self::ESIZE_F)) | (block2 << (p1 * Self::ESIZE_F)) |(block3 << (p4 * Self::ESIZE_F)) |(block4 << (p3 * Self::ESIZE_F)));
        edges
    }

    #[inline(always)]
    fn base_double_c(&mut self, p1: usize, p2: usize, p3: usize, p4: usize, mask: Self::Corner) -> Self::Corner{
        let blank_c = (Self::Corner::try_from(2).ok().unwrap() << (Self::CSIZE_F-1) )-Self::Corner::try_from(1).ok().unwrap();
        let mut corners = *self.corner();
        let block1 = (corners >> (p1 * Self::CSIZE_F)) & blank_c;
        let block2 = (corners >> (p2 * Self::CSIZE_F)) & blank_c;
        let block3 = (corners >> (p3 * Self::CSIZE_F)) & blank_c;
        let block4 = (corners >> (p4 * Self::CSIZE_F)) & blank_c;

        corners =  corners & mask; 
        corners = corners ^ ((block1 << (p2 * Self::CSIZE_F)) | (block2 << (p1 * Self::CSIZE_F)) |(block3 << (p4 * Self::CSIZE_F)) |(block4 << (p3 * Self::CSIZE_F)));
        corners
    }

    fn r(mut self) -> Self {
        *self.edge() = self.base_move_e(Self::DR,Self::FR,Self::UR,Self::BR, Self::Edge::try_from(Self::RE_MASK).ok().unwrap(), false);
        *self.corner() = self.base_move_c(Self::DFR,Self::UFR,Self::UBR,Self::DBR, Self::Corner::try_from(Self::RC_MASK).ok().unwrap(), true);
        self
    }

    fn rp(mut self) -> Self {
        *self.edge() = self.base_move_e(Self::DR,Self::BR,Self::UR,Self::FR, Self::Edge::try_from(Self::RE_MASK).ok().unwrap(), false);
        *self.corner() = self.base_move_c(Self::DFR,Self::DBR,Self::UBR,Self::UFR, Self::Corner::try_from(Self::RC_MASK).ok().unwrap(), true);
        self
    }

    fn r2(mut self) -> Self {
        *self.edge() = self.base_double_e(Self::DR,Self::UR,Self::FR,Self::BR, Self::Edge::try_from(Self::RE_MASK).ok().unwrap());
        *self.corner() = self.base_double_c(Self::DFR,Self::UBR,Self::UFR,Self::DBR, Self::Corner::try_from(Self::RC_MASK).ok().unwrap());
        self
    }

    fn l(mut self) -> Self {
        *self.edge() = self.base_move_e(Self::DL,Self::BL,Self::UL,Self::FL, Self::Edge::try_from(Self::LE_MASK).ok().unwrap(), false);
        *self.corner() = self.base_move_c(Self::UFL, Self::DFL,Self::DBL,Self::UBL, Self::Corner::try_from(Self::LC_MASK).ok().unwrap(), true);
        self
    }

    fn lp(mut self) -> Self {
        *self.edge() = self.base_move_e(Self::DL,Self::FL,Self::UL,Self::BL, Self::Edge::try_from(Self::LE_MASK).ok().unwrap(), false);
        *self.corner() = self.base_move_c(Self::UFL, Self::UBL,Self::DBL,Self::DFL, Self::Corner::try_from(Self::LC_MASK).ok().unwrap(), true);
        self
    }

    fn l2(mut self) -> Self {
        *self.edge() = self.base_double_e(Self::DL,Self::UL,Self::FL,Self::BL, Self::Edge::try_from(Self::LE_MASK).ok().unwrap());
        *self.corner() = self.base_double_c(Self::DFL,Self::UBL,Self::UFL,Self::DBL, Self::Corner::try_from(Self::LC_MASK).ok().unwrap());
        self
    }

    fn u(mut self) -> Self {
        *self.edge() = self.base_move_e(Self::UF,Self::UL,Self::UB,Self::UR, Self::Edge::try_from(Self::UE_MASK).ok().unwrap(), false);
        *self.corner() = self.base_move_c(Self::UFL, Self::UBL,Self::UBR,Self::UFR, Self::Corner::try_from(Self::UC_MASK).ok().unwrap(), false);
        self
    }

    fn up(mut self) -> Self {
        *self.edge() = self.base_move_e(Self::UF,Self::UR,Self::UB,Self::UL, Self::Edge::try_from(Self::UE_MASK).ok().unwrap(), false);
        *self.corner() = self.base_move_c(Self::UFL, Self::UFR,Self::UBR,Self::UBL, Self::Corner::try_from(Self::UC_MASK).ok().unwrap(), false);
        self
    }

    fn u2(mut self) -> Self {
        *self.edge() = self.base_double_e(Self::UR,Self::UL,Self::UF,Self::UB, Self::Edge::try_from(Self::UE_MASK).ok().unwrap());
        *self.corner() = self.base_double_c(Self::UFL,Self::UBR,Self::UFR,Self::UBL, Self::Corner::try_from(Self::UC_MASK).ok().unwrap());
        self
    }

    fn d(mut self) -> Self {
        *self.edge() = self.base_move_e(Self::DF,Self::DR,Self::DB,Self::DL, Self::Edge::try_from(Self::DE_MASK).ok().unwrap(), false);
        *self.corner() = self.base_move_c(Self::DFL, Self::DFR,Self::DBR,Self::DBL, Self::Corner::try_from(Self::DC_MASK).ok().unwrap(), false);
        self
    }

    fn dp(mut self) -> Self {
        *self.edge() = self.base_move_e(Self::DF,Self::DL,Self::DB,Self::DR, Self::Edge::try_from(Self::DE_MASK).ok().unwrap(), false);
        *self.corner() = self.base_move_c(Self::DFL, Self::DBL,Self::DBR,Self::DFR, Self::Corner::try_from(Self::DC_MASK).ok().unwrap(), false);
        self
    }

    fn d2(mut self) -> Self {
        *self.edge() = self.base_double_e(Self::DF,Self::DB,Self::DL,Self::DR, Self::Edge::try_from(Self::DE_MASK).ok().unwrap());
        *self.corner() = self.base_double_c(Self::DFL,Self::DBR,Self::DFR,Self::DBL, Self::Corner::try_from(Self::DC_MASK).ok().unwrap());
        self
    }

    fn f(mut self) -> Self {
        *self.edge() = self.base_move_e(Self::DF,Self::FL,Self::UF,Self::FR, Self::Edge::try_from(Self::FE_MASK).ok().unwrap(), true);
        *self.corner() = self.base_move_c(Self::DFL, Self::UFL,Self::UFR,Self::DFR, Self::Corner::try_from(Self::FC_MASK).ok().unwrap(), true);
        self
    }

    fn fp(mut self) -> Self {
        *self.edge() = self.base_move_e(Self::DF,Self::FR,Self::UF,Self::FL, Self::Edge::try_from(Self::FE_MASK).ok().unwrap(), true);
        *self.corner() = self.base_move_c(Self::DFL, Self::DFR,Self::UFR,Self::UFL, Self::Corner::try_from(Self::FC_MASK).ok().unwrap(), true);
        self
    }

    fn f2(mut self) -> Self {
        *self.edge() = self.base_double_e(Self::DF,Self::UF,Self::FR,Self::FL, Self::Edge::try_from(Self::FE_MASK).ok().unwrap());
        *self.corner() = self.base_double_c(Self::DFL,Self::UFR,Self::DFR,Self::UFL, Self::Corner::try_from(Self::FC_MASK).ok().unwrap());
        self
    }

    fn b(mut self) -> Self {
        *self.edge() = self.base_move_e(Self::UB,Self::BL,Self::DB,Self::BR, Self::Edge::try_from(Self::BE_MASK).ok().unwrap(), true);
        *self.corner() = self.base_move_c(Self::DBR, Self::UBR, Self::UBL,Self::DBL, Self::Corner::try_from(Self::BC_MASK).ok().unwrap(), true);
        self
    }

    fn bp(mut self) -> Self {
        *self.edge() = self.base_move_e(Self::UB,Self::BR,Self::DB,Self::BL, Self::Edge::try_from(Self::BE_MASK).ok().unwrap(), true);
        *self.corner() = self.base_move_c(Self::DBR, Self::DBL, Self::UBL,Self::UBR, Self::Corner::try_from(Self::BC_MASK).ok().unwrap(), true);
        self
    }

    fn b2(mut self) -> Self {
        *self.edge() = self.base_double_e(Self::DB,Self::UB,Self::BR,Self::BL, Self::Edge::try_from(Self::BE_MASK).ok().unwrap());
        *self.corner() = self.base_double_c(Self::DBR,Self::UBL,Self::UBR,Self::DBL, Self::Corner::try_from(Self::BC_MASK).ok().unwrap());
        self
    }

    fn perform_move(mut self, movee: u8) -> Self{
        self = match movee{
            1 => self.r(),
            2 => self.l(),
            3 => self.u(),
            4 => self.d(),
            5 => self.f(),
            6 => self.b(),
            11 => self.r2(),
            12 => self.l2(),
            13 => self.u2(),
            14 => self.d2(),
            15 => self.f2(),
            16 => self.b2(),
            21 => self.rp(),
            22 => self.lp(),
            23 => self.up(),
            24 => self.dp(),
            25 => self.fp(),
            26 => self.bp(),
            0 => self,
            _ => unreachable!()
        };
        self
    }

    fn do_scramble(mut self, scramble:String) -> (Self,Vec<u8>) {
        let split_scramble: Vec<_> = scramble.split_ascii_whitespace().map(|f|
            match f {
                "R" => 1,
                "L" => 2,
                "U" => 3,
                "D" => 4,
                "F" => 5,
                "B" => 6,
                "R2" => 11,
                "L2" => 12,
                "U2" => 13,
                "D2" => 14,
                "F2" => 15,
                "B2" => 16,
                "R'" => 21,
                "L'" => 22,
                "U'" => 23,
                "D'" => 24,
                "F'" => 25,
                "B'" => 26,
                _ => unreachable!()
            }).collect();
    
        for movei in &split_scramble{
            self = self.perform_move(*movei);
        }
        (self, split_scramble)
    }
    
}

#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq,Ord,PartialOrd)]
pub struct Cube{
    corners: u64,
    edges: u64,
}

impl BaseCube for Cube{
    const ESIZE: usize = 4;
    const CSIZE: usize = 3;
    const CORNER_ORINENTATION: bool = true;
    const EDGE_ORINENTATION: bool = true;

    type Corner = u64;
    type Edge = u64;

    fn edge(&mut self) ->  &mut Self::Edge {
        &mut self.edges
    }

    fn corner(&mut self) -> &mut Self::Corner {
        &mut self.corners
    }

    fn new() -> Self{
        Self {
            corners: 247132686368,
            edges: 407901468851537952,
        }
    }
}

#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq,Ord,PartialOrd)]
pub struct DRToHTRCube{
    corners: u32,
    edges: u16,
}

impl BaseCube for DRToHTRCube{
    const ESIZE: usize = 1;
    const CSIZE: usize = 3;
    const CORNER_ORINENTATION: bool = false;
    const EDGE_ORINENTATION: bool = false;

    type Corner = u32;
    type Edge = u16;

    fn edge(&mut self) ->  &mut Self::Edge {
        &mut self.edges
    }

    fn corner(&mut self) -> &mut Self::Corner {
        &mut self.corners
    }

    fn new() -> Self{
        Self {
            corners: 16434824,
            edges: 2313,
        }
    }
}

impl DRToHTRCube{

    pub fn construct() -> Self { // The way the numbers in new is calculated
        let mut c = 0;
        for i in (0..8).rev(){
            c = c << Self::CSIZE_F;
            // if [Self::DBL,Self::DFR,Self::UFL,Self::UBR].contains(&i){
            //     c += 1
            // }
            c += i;
        }
        let mut e = 0;
        for i in (0..12).rev(){
            e = e << Self::ESIZE_F;
            if [Self::UF,Self::UB,Self::DF,Self::DB].contains(&i){
                e += 1
            }
        }

        Self {
            corners: c,
            edges: e
        }
    }

    pub fn edges_minus(self) -> u32{
        let overlap = self.edges & Self::new().edges;
        8 - (overlap.count_ones()*2)
    }

    pub fn corners_minus(self) -> u32{
        // let overlap = *self.corner() & Self::new().corners;
        // 8 - (overlap.count_ones()*2)
        let mut count = 0;
        for val in [Self::DBL,Self::DFR,Self::UFL,Self::UBR]{
            if [Self::DBL,Self::DFR,Self::UFL,Self::UBR].contains(&(((self.corners >> (val * Self::CSIZE_F)) & 7) as usize)){
                count +=1;
            }
        }
        8-(count*2)
    }
}


#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq,Ord,PartialOrd)]
pub struct EOToDRCube{
    corners: u32,
    edges: u16,
}

impl BaseCube for EOToDRCube{
    const ESIZE: usize = 1;
    const CSIZE: usize = 1; // If the code could handle 0 size, it would be better
    const CORNER_ORINENTATION: bool = true;
    const EDGE_ORINENTATION: bool = false;

    type Corner = u32;
    type Edge = u16;

    fn edge(&mut self) ->  &mut Self::Edge {
        &mut self.edges
    }

    fn corner(&mut self) -> &mut Self::Corner {
        &mut self.corners
    }

    fn new() -> Self{
        Self { 
            corners: 0,
            edges: 240,
        }
    }
}

impl EOToDRCube{

    pub fn construct() -> Self { // The way the numbers in new is calculated
        let c = 0;

        let mut e = 0;
        for i in (0..12).rev(){
            e = e << Self::ESIZE_F;
            if [Self::FR,Self::FL,Self::BL,Self::BR].contains(&i){
                e += 1
            }
        }

        Self {
            corners: c,
            edges: e
        }
    }

    pub fn edges_minus(self) -> u32{ //TODO
        let overlap = self.edges & Self::new().edges;
        8 - (overlap.count_ones()*2)
    }

    pub fn corners_minus(self) -> u32{
        // let overlap = *self.corner() & Self::new().corners;
        // 8 - (overlap.count_ones()*2)
        self.corners.count_ones()
    }
}

#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq,Ord,PartialOrd)]
pub struct EOOnlyCube{
    corners: u32,
    edges: u32,
}

impl BaseCube for EOOnlyCube{
    const ESIZE: usize = 1; // If the code could handle 0 size, it would be better
    const CSIZE: usize = 1; 
    const CORNER_ORINENTATION: bool = false;
    const EDGE_ORINENTATION: bool = true;

    type Corner = u32;
    type Edge = u32;

    fn edge(&mut self) ->  &mut Self::Edge {
        &mut self.edges
    }

    fn corner(&mut self) -> &mut Self::Corner {
        &mut self.corners
    }

    fn new() -> Self{
        Self { 
            corners: 0,
            edges: 0,
        }
    }
}

impl EOOnlyCube{

    pub fn construct() -> Self { // The way the numbers in new is calculated
        Self {
            corners: 0,
            edges: 0
        }
    }

    pub fn un_oriented_edges(self) -> u32{
        self.edges.count_ones()
    }
}