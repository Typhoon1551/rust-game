# rust-game
A terminal game engine written in rust

## Usage
+ The basic screen renderer item is a `Scene`
  - Scenes are made up of one `Surface`
    - `Surface`s contain an array of `char`s, which they render directly to the terminal
  - And a vector of `Layer`s
    - A `Layer` contains an array of `bool`s and a single `char` value
    - `Layer`s render onto a surface, by insertinng its character based on the boolean array
+ When a scenes render call is made, it first renders all or the layers onto the surface, in order of their index in the vector. It then renders the surface to the terminal