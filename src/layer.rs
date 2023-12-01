use rand::random;
use crate::surface::Surface;

pub struct Layer {
   map_: Vec<Vec<bool>>,
   char_: char,
   width_: usize,
   height_: usize,
}

impl Layer {
   pub fn new(char_: char, w:usize, h:usize) -> Layer {
       return Layer {
           map_: vec![vec![false;w];h],
           char_,
           width_: w,
           height_: h,
       }
   }

   pub fn random(char_: char, w:usize, h:usize) -> Layer {
       let mut map_ = vec![vec![false;w];h];

       for y in 0..h {
           for x in 0..w {
               map_[y][x] = random();
           }
       }
       
       return Layer {
           map_,
           char_,
           width_: w,
           height_: h,
       }
   }

   pub fn from(map_: Vec<Vec<bool>>, char_: Option<char>) -> Layer{
       return Layer {
           map_ : map_.clone(),
           char_: char_.unwrap_or('#'),
           width_: map_[0].len(),
           height_: map_.len(),
       }
   }

   pub fn add_to_surface(&self, screen: &mut Surface) -> u32 {
       for y in 0..self.height_ {
           for x in 0..self.width_ {
               if self.map_[y][x] {
                   screen.set((x, y), self.char_);
               }
           }
       }
       return 0;
   }

   pub fn set (&mut self, pos: (usize, usize), val:bool) -> u32 {
       self.map_[pos.1][pos.0] = val;
       return 0;
   }

   pub fn get (&self, pos : (usize, usize)) -> bool {
       return self.map_[pos.1][pos.0];
   }
}