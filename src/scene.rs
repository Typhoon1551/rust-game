use crate::surface::Surface;
use crate::layer::Layer;

pub struct ConstructLayer(pub char, pub String);

pub struct Scene {
   surface_ : Surface,
   layers_ : Vec<Layer>,
   names_ : Vec<String>,
   size_ : (usize,usize),
}

impl Scene {
   pub fn new(size: (usize,usize)) -> Scene {
      return Scene {
         surface_ : Surface::new(size, None),
         layers_ : vec![Layer::non();1],
         names_ : vec![String::new();1],
         size_ : size,
      }
   }

   pub fn insert_layer(&mut self, idx : usize, layer_ : ConstructLayer) -> i32 {
      let len = self.layers_.len();
      self.layers_.insert(idx%len, Layer::new(layer_.0, self.size_.0, self.size_.1));
      self.names_.insert(idx%len, layer_.1);
      return 0;
   }

   pub fn set_layer (&mut self, idx : usize, layer : Layer, name : String) {
      let len = self.layers_.len();
      self.layers_[idx%len] = layer;
      self.names_[idx%len] = name;
   }

   pub fn get_layer (&mut self, idx : usize) -> &mut Layer {
      let len = self.layers_.len();
      return &mut self.layers_[idx%len];
   }

   pub fn get_layer_by_name (&mut self, name : String) -> &mut Layer {
      for (i,n) in self.names_.iter_mut().enumerate() {
         if n.to_owned() == name {
            return &mut self.layers_[i];
         }
      }
      return Layer::non();
   }

   pub fn get_layer_index (&mut self, name : String) -> usize {
      for (i,n) in self.names_.iter_mut().enumerate() {
         if n.to_owned() == name {
            return i;
         }
      }
      return 0;
   }

   pub fn render (&mut self) -> i32 {
      self.surface_.clear();
      for layer in self.layers_.iter_mut() {
         layer.add_to_surface(&mut self.surface_);
      }
      self.surface_.render();
      return 0;
   }
}