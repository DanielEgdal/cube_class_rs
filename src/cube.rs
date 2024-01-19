#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)] 
pub struct Cube {
    pub corners: u64,
    pub edges: u64,
}
trait BaseCube {
    fn u(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        let block1 = (c >> 20) & 31;
        let block2 = (c >> 25) & 31;
        let block3 = (c >> 30) & 31;
        let block4 = (c >> 35) & 31;
    
        c = c & 34084861509631;
        c = c ^ (( block1 << 25) | ( block2 << 35) |( block3 << 20 ) |( block4 << 30));
    
        let block1 = (e >> 40) & 31;
        let block2 = (e >> 45) & 31;
        let block3 = (e >> 50) & 31;
        let block4 = (e >> 55) & 31;
        e = e & 1099511627775;
        e = e ^ (( block1 << 45) | ( block2 << 55) |( block3 << 40) |( block4 << 50));
    
        self.edges = e;
        self.corners = c;
        
        self
    }

}

impl BaseCube for Cube{

}

impl Cube {

    pub fn new() -> Self{
        Self {
            corners: 247132686368,
            edges: 407901468851537952,
        }
    }

    fn swap5(self, k: u64, pos1: u8, pos2: u8) -> u64{
        let set1 =  (k >> pos1) & 31;
        let set2 = (k >> pos2) & 31;
        let mut xor = set1 ^ set2;
        xor = (xor << pos1) | (xor << pos2);
        let ret_val = k ^ xor;
        return ret_val;
    }

    fn twist_corner(self, k:u64) -> u64{
        let u = k & 24;
        let o = (u+8+((u&16)>>1))&24;
        return o + (k&7)
    }

    fn twist_corner_c(self, k:u64) -> u64{

        let u = k & 24;
        let t = u + 24;
        let o = (t - ((t & 16)>>1)) & 24;
        return o + (k&7)
    }

    pub fn u(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        let block1 = (c >> 20) & 31;
        let block2 = (c >> 25) & 31;
        let block3 = (c >> 30) & 31;
        let block4 = (c >> 35) & 31;
    
        c = c & 34084861509631;
        c = c ^ (( block1 << 25) | ( block2 << 35) |( block3 << 20 ) |( block4 << 30));
    
        let block1 = (e >> 40) & 31;
        let block2 = (e >> 45) & 31;
        let block3 = (e >> 50) & 31;
        let block4 = (e >> 55) & 31;
        e = e & 1099511627775;
        e = e ^ (( block1 << 45) | ( block2 << 55) |( block3 << 40) |( block4 << 50));
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    pub fn up(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;
        let block1 = (c >> 20) & 31;
        let block2 = (c >> 25) & 31;
        let block3 = (c >> 30) & 31;
        let block4 = (c >> 35) & 31;
    
        c = c & 34084861509631;
        c = c ^  ((block1 << 30) | (block2 << 20) |(block3 << 35 ) |(block4 << 25));
    
        let block1 = (e >> 40) & 31;
        let block2 = (e >> 45) & 31;
        let block3 = (e >> 50) & 31;
        let block4 = (e >> 55) & 31;
        e = e & 1099511627775;
        e = e ^ ((block1 << 50) | (block2 << 40) |(block3 << 55) |(block4 << 45));
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn d(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        let block1 = (c) & 31;
        let block2 = (c >> 5) & 31;
        let block3 = (c >> 10) & 31;
        let block4 = (c >> 15) & 31;
    
        c = c & 35184371040256;
        c = c ^ ((block1 << 5) | (block2 << 15) |(block3) |(block4 << 10));
    
        let block1 = (e) & 31;
        let block2 = (e >> 5) & 31;
        let block3 = (e >> 10) & 31;
        let block4 = (e >> 15) & 31;
        e = e & 1152921504605798400;
        e = e ^ ((block1 << 5) | (block2 << 15) |(block3) |(block4 << 10));
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn dp(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        let block1 = (c) & 31;
        let block2 = (c >> 5) & 31;
        let block3 = (c >> 10) & 31;
        let block4 = (c >> 15) & 31;
    
        c = c & 35184371040256;
        c = c ^ ((block1 << 10) | (block2) |(block3 << 15) |(block4 << 5));
    
        let block1 = (e) & 31;
        let block2 = (e >> 5) & 31;
        let block3 = (e >> 10) & 31;
        let block4 = (e >> 15) & 31;
        e = e & 1152921504605798400;
        e = e ^ ((block1 << 10) | (block2) |(block3 << 15) |(block4 << 5));
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn f(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        let block1 = self.twist_corner((c >> 10) & 31);
        let block2 = self.twist_corner_c((c >> 15) & 31);
        let block3 = self.twist_corner_c((c >> 20) & 31);
        let block4 = self.twist_corner((c >> 25) & 31);
        c = c & 35183298348031;
        c = c ^ ((block1 << 15) | (block2 << 25) |(block3 << 10) |(block4 << 20));
    
        let block1 = ((e >> 15) & 31)^16;
        let block2 = ((e >> 30) & 31) ^16;
        let block3 = ((e >> 35) & 31) ^16;
        let block4 = ((e >> 40) & 31) ^16;
        e = e & 1152886321307484159;
        e = e ^ ((block1 << 35) | (block2 << 15) |(block3 << 40) |(block4 << 30));
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn fp(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        let block1 = self.twist_corner((c >> 10) & 31);
        let block2 = self.twist_corner_c((c >> 15) & 31);
        let block3 = self.twist_corner_c((c >> 20) & 31);
        let block4 = self.twist_corner((c >> 25) & 31);
        c = c & 35183298348031;
        c = c ^ ((block1 << 20) | (block2 << 10) |(block3 << 25) |(block4 << 15));
    
        let block1 = ((e >> 15) & 31)^16;
        let block2 = ((e >> 30) & 31) ^16;
        let block3 = ((e >> 35) & 31) ^16;
        let block4 = ((e >> 40) & 31) ^16;
        e = e & 1152886321307484159;
        e = e ^ ((block1 << 30) | (block2 << 40) |(block3 << 15) |(block4 << 35));
        self.edges = e;
        self.corners = c;
        
        self
    
    }
    
    pub fn l(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        let block1 = self.twist_corner_c((c >> 5) & 31);
        let block2 = self.twist_corner((c >> 15) & 31);
        let block3 = self.twist_corner_c((c >> 25) & 31);
        let block4 = self.twist_corner((c >>35) & 31);
        c = c & 34118178995231;
        c = c ^ ((block1 << 35) | (block2 << 5) |(block3 << 15) |(block4 << 25));
    
        let block1 = (e >> 5) & 31;
        let block2 = (e >> 20) & 31;
        let block3 = (e >> 35) & 31;
        let block4 = (e >> 45) & 31;
        e = e & 1151829723887696927;
        e = e ^ ((block1 << 20) | (block2 << 45) |(block3 << 5) |(block4 << 35));
    
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn lp(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        let block1 = self.twist_corner_c((c >> 5) & 31);
        let block2 = self.twist_corner((c >> 15) & 31);
        let block3 = self.twist_corner_c((c >> 25) & 31);
        let block4 = self.twist_corner((c >>35) & 31);
        c = c & 34118178995231;
        c = c ^ ((block1 << 15) | (block2 << 25) |(block3 << 35) |(block4 << 5));
    
        let block1 = (e >> 5) & 31;
        let block2 = (e >> 20) & 31;
        let block3 = (e >> 35) & 31;
        let block4 = (e >> 45) & 31;
        e = e & 1151829723887696927;
        e = e ^ ((block1 << 35) | (block2 << 5) |(block3 << 45) |(block4 << 20));
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn r(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;
    
        let block1 = self.twist_corner_c((c >> 10) & 31);
        let block2 = self.twist_corner((c >> 20) & 31);
        let block3 = self.twist_corner_c((c >> 30) & 31);
        let block4 = self.twist_corner((c) & 31);
        c =  c & 35151053554656;
        c = c ^ ((block1 << 20) | (block2 << 30) |(block3) |(block4 << 10));
    
        let block1 = (e >> 10) & 31;
        let block2 = (e >> 25) & 31;
        let block3 = (e >> 30) & 31;
        let block4 = (e >> 50) & 31;
        e = e & 1118018573168509951;
        e = e ^ ((block1 << 30) | (block2 << 10) |(block3 << 50) |(block4 << 25));
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn rp(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        let block1 = self.twist_corner_c((c >> 10) & 31);
        let block2 = self.twist_corner((c >> 20) & 31);
        let block3 = self.twist_corner_c((c >> 30) & 31);
        let block4 = self.twist_corner((c) & 31);
        c =  c & 35151053554656;
        c = c ^ ((block1) | (block2 << 10) |(block3 << 20) |(block4 << 30));
    
        let block1 = (e >> 10) & 31;
        let block2 = (e >> 25) & 31;
        let block3 = (e >> 30) & 31;
        let block4 = (e >> 50) & 31;
        e = e & 1118018573168509951;
        e = e ^ ((block1 << 25) | (block2 << 50) |(block3 << 10) |(block4 << 30));
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn b(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        let block1 = self.twist_corner_c((c) & 31);
        let block2 = self.twist_corner((c >> 5) & 31);
        let block3 = self.twist_corner((c >> 30) & 31);
        let block4 = self.twist_corner_c((c >> 35) & 31);
        c = c & 34085934201856;
        c = c ^ ((block1 << 30) | (block2) |(block3 << 35) |(block4 << 5));
    
        let block1 = ((e) & 31)^16;
        let block2 = ((e >> 20) & 31) ^16;
        let block3 = ((e >> 25) & 31) ^16;
        let block4 = ((e >> 55) & 31) ^16;
        e = e & 36028795946270688;
        e = e ^ ((block1 << 25) | (block2) |(block3 << 55) |(block4 << 20));
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn bp(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        let block1 = self.twist_corner_c((c) & 31);
        let block2 = self.twist_corner((c >> 5) & 31);
        let block3 = self.twist_corner((c >> 30) & 31);
        let block4 = self.twist_corner_c((c >> 35) & 31);
        c = c & 34085934201856;
        c = c ^ ((block1 << 5) | (block2 << 35) |(block3 ) |(block4 << 30));
    
        let block1 = ((e) & 31)^16;
        let block2 = ((e >> 20) & 31) ^16;
        let block3 = ((e >> 25) & 31) ^16;
        let block4 = ((e >> 55) & 31) ^16;
        e = e & 36028795946270688;
        e = e ^ ((block1 << 20) | (block2 << 55) |(block3) |(block4 << 25));
    
        self.edges = e;
        self.corners = c;
        
        self
    }

    pub fn u2(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        c = self.swap5(c,20,35);
        c = self.swap5(c,30,25);
    
        e = self.swap5(e,40,55);
        e = self.swap5(e,45,50);
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn d2(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        c = self.swap5(c,0,15);
        c = self.swap5(c,5,10);
    
        e = self.swap5(e,0,15);
        e = self.swap5(e,5,10);
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn f2(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        c = self.swap5(c,20,15);
        c = self.swap5(c,25,10);
    
        e = self.swap5(e,15,40);
        e = self.swap5(e,30,35);
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn l2(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        c = self.swap5(c,35,15);
        c = self.swap5(c,25,5);
    
        e = self.swap5(e,45,5);
        e = self.swap5(e,35,20);
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn b2(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        c = self.swap5(c,30,5);
        c = self.swap5(c,35,0);
    
        e = self.swap5(e,25,20);
        e = self.swap5(e,55,0);
    
        self.edges = e;
        self.corners = c;
        
        self
    }
    
    pub fn r2(mut self)-> Self{
        let mut e = self.edges;
        let mut c = self.corners;

        c = self.swap5(c,10,30);
        c = self.swap5(c,20,0);
    
        e = self.swap5(e,10,50);
        e = self.swap5(e,30,25);
    
        self.edges = e;
        self.corners = c;
        
        self
    }

    pub fn perform_move(mut self, movee: u8) -> Self {
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

    pub fn do_scramble(mut self, scramble:String) -> (Self,Vec<u8>){
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

    pub fn is_solved(self) -> bool{
        (self.corners == 247132686368) && (self.edges == 407901468851537952)
    }
}

// fn main(){
//     let mut c = Cube::new();
//     // let (mut z, scramble_vec) = c.do_scramble("R' U' F R2 B R2 D B L F' R D2 R' U2 L' F2 R' D2 F2 R B U' R' U' F".to_string());
//     // for i in 0..10000000 {
//     //     let (new_z, scramble_vec2) = z.do_scramble("F2 U' F' U' L R2 U B R' D2 R U2 R' D' B D' B R' B2 R B U R'".to_string());
//     //     z = new_z;
//     // }
//     for i in 0..1_000_000_001 {
//         let z = c.f();
//         c = z;
//     }
//     println!("{:?}",c);

// }