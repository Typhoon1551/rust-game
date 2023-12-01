use crate::surface::Surface;
use crate::layer::Layer;

pub struct Scene {
   surface_ : Surface,
   layers_ : Vec<Layer>,
   player_ : (usize,usize,char),
   size_ : (usize,usize),
}

impl Scene {
   pub fn new(size: (usize,usize), player_char: char) -> Scene {
      return Scene {
         surface_ : Surface::new(size, None),
         layers_ : vec![Layer::non();0],
         player_ : (0,0,player_char),
         size_ : size,
      }
   }

   pub fn add_layer(&mut self, idx : usize, char_: char, name : String) -> i32 {
      let len = self.layers_.len();
      self.layers_[idx%len] = Layer::new(char_, self.size_.0, self.size_.1, name);
      return 0;
   }
}