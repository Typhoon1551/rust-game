use rand::random;
use crate::surface::Surface;


#[derive(Clone)]
pub struct Layer {
   map_: Vec<Vec<bool>>,
   char_: char,
   width_: usize,
   height_: usize,
}

impl Layer {
   pub fn new(char_: char, width:usize, height:usize) -> Layer {
       return Layer {
           map_: vec![vec![false;width];height],
           char_,
           width_: width,
           height_: height,
       }
   }

   pub fn random(&mut self) -> i32 {
      for y in 0..self.height_ {
           for x in 0..self.width_ {
               self.map_[y][x] = random();
           }
      }
       return 0;
   }

   pub fn non() -> Layer {
       return Layer {
           map_: vec![vec![false;0];0],
           char_: ' ',
           width_: 0,
           height_: 0,
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

   pub fn clear (&mut self) -> u32 {
       for y in 0..self.height_ {
           for x in 0..self.width_ {
               self.map_[y][x] = false;
           }
       }
       return 0;
   }
}