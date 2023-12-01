use rand::random;
use crate::surface::Surface;


#[derive(Clone)]
pub struct Layer {
   map_: Vec<Vec<bool>>,
   char_: char,
   width_: usize,
   height_: usize,
   name_: String,
}

impl Layer {
   pub fn new(char_: char, width:usize, height:usize, name : String) -> Layer {
       return Layer {
           map_: vec![vec![false;width];height],
           char_,
           width_: width,
           height_: height,
           name_: name,
       }
   }

   pub fn random(char_: char, w:usize, h:usize, name : String) -> Layer {
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
           name_ : name,
       }
   }

   pub fn from(map_: Vec<Vec<bool>>, char_: Option<char>, name : String) -> Layer{
       return Layer {
           map_ : map_.clone(),
           char_: char_.unwrap_or('#'),
           width_: map_[0].len(),
           height_: map_.len(),
           name_ : name,
       }
   }

   pub fn non() -> Layer {
       return Layer {
           map_: vec![vec![false;0];0],
           char_: ' ',
           width_: 0,
           height_: 0,
           name_: String::from(""),
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