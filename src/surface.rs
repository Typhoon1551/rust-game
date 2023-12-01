pub struct Surface {
   surface_: Vec<Vec<char>>,
   fill_: char,
}

impl Surface {
   pub fn new(size : (usize,usize), fill: Option<char>) -> Surface{
       return Surface { 
           surface_: vec![vec![fill.unwrap_or(' ');size.0];size.1],
           fill_: fill.unwrap_or(' '),
       };
   }

   pub fn get(&self, pos : (usize, usize)) -> char {
       return self.surface_[pos.1][pos.0];
   }

   pub fn set(&mut self, pos : (usize,usize), fill: char) -> u32{
       self.surface_[pos.1][pos.0] = fill;
       return 0;
   }

   pub fn render(&self) -> u32{
       println!("{}{}{}",'+',"-".repeat(100),'+');
       for y in self.surface_.iter() {
           println!("|{}|",y.iter().collect::<String>());
       }
       println!("{}{}{}",'+',"-".repeat(100),'+');
       return 0;
   }

   pub fn clear(&mut self) -> u32{
      self.surface_ = vec![vec![self.fill_;100];10];
      return 0;
   }
}