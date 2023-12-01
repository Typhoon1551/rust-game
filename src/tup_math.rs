pub fn tup_u32_to_usize(tup: (u32, u32)) -> (usize, usize) {
   return (tup.0 as usize, tup.1 as usize);
}

pub fn tup_i32_to_usize(tup: (i32, i32)) -> (usize, usize) {
   return (tup.0 as usize, tup.1 as usize);
}

pub fn tup_usize_to_i32(tup: (usize, usize)) -> (i32, i32) {
   return (tup.0 as i32, tup.1 as i32);
}

pub fn tup_usize_to_u32(tup: (usize, usize)) -> (u32, u32) {
   return (tup.0 as u32, tup.1 as u32);
}

pub fn tup_i32_clamp(val: (i32,i32), min: (i32,i32), max: (i32,i32)) -> (i32,i32) {
   let a = {
       if val.0 < min.0 {
           min.0
       } else if val.0 >= max.0 {
           max.0 - 1
       } else {
           val.0
       }
   };
   let b = {
       if val.1 < min.1 {
           min.1
       } else if val.1 >= max.1 {
           max.1 - 1
       } else {
           val.1
       }
   };
   return (a,b);
}