# w12\_final
Members: Arif Roktim Period 4  
Team Name: K***rust***y Krab

## Running scripts
All mdl files are under `scripts/`  
To run a script, do `$ make S=<path to script>` The script file can be tab completed  
Example: `$ make S=scripts/teapot.mdl`

## Implemented Features

### Existing MDL Commands/features:
- [x] Added `light` mdl command; I loop through all the lights  
- [x] Add `mesh` mdl command  
Demonstration: [scripts/teapot\_colorful\_moving.mdl](scripts/teapot_colorful_moving.mdl)  
![colorfulmovingteapot](http://gallery.stuycs.org/static/images/2019/scale/1139.GIF)

---

### Time to up my compiler game:
- [x] Do arithmetic on expressions

Expressions are always contained in parentheses. The available operators are
- `+`:  Addition
- `-`:  Subtraction
- `*`:  Multiplication
- `/`:  Division
- `//`: Integer Division

The type casting works like python 2:  
For the first 4 operators, if either of the arguments is a float, the output is a float.
Otherwise, the output is an integer.
The output of integer division is always an integer.

The operators are evaluated according to the order of operations:
- Multiplication, Divison, and Integer Divison, all have the highest precedence.
- Addition and subtraction have lower precedence.
- Operators with same precedence are evaluated from left to right

Demonstration: [scripts/expr.mdl](scripts/expr.mdl)
```
-- snip --
# Demonstration of expression evaluation
# Order of operations is respected:
# (250 + 1.0 / 2) => 250.5, so image is vertically centered
# If order of operations WASN'T respected, the above would evaluate to 125.5
# so image wouldn't be vertically centered
move (0 + 250) (250 + 1 / 2) 0
sphere shiny_purple -100 150 0 80
sphere shiny_teal 100 150 0 80
-- snip --
```

![exprtorus](https://i.ibb.co/wMjjSkM/expr.png)

---

- [x] Add ability to reference variables in expressions

Instead of manually changing the `start` and `end` frames for the `vary` command
when you change the number of frames, you can instead specify the arguments in
terms of the number of frames.

Demonstration: [scripts/var.mdl](scripts/var.mdl)
```
frames 50
basename simple_torus_var
push
move (XRES / 3) (YRES / 2) 0
scale 2 2 2 bigenator
rotate y 360 spinny
rotate z 360 spinny
torus 0 0 0 75 125
vary spinny 0 (FRAMES - 1) 0 1
vary bigenator 0 (FRAMES / 2 - 1) 0 1
vary bigenator (FRAMES / 2) (FRAMES - 1) 1 0
```

![torus](https://thumbs.gfycat.com/HeavenlyLastingJerboa-size_restricted.gif)

This isn't limited to the `vary` command.  For example, the teapot mesh file 
is very very small, so it has to be scaled up. You can scale the teapot based
on the screen resolution.

Demonstration: [scripts/teapot\_colorful\_moving\_vars.mdl](scripts/teapot_colorful_moving_vars.mdl)
```
# scale the X and Y axis by a ratio of the resolutions
# We don't have a Z resolution for the screen 
# so I took the average of the X and Y scales
scale (XRES / 7.5) (YRES / 7.5) ((XRES / 7.5 + YRES / 7.5) / 2)
mesh :teapot.obj
```

800x800 gif  
![largeshinyteapot](https://thumbs.gfycat.com/RectangularBestDog-size_restricted.gif)

If you're interested in trying out different resolutions, modify lines `16` and `17`
of `graphics/src/main.rs`

```rust
let mut screen = ScreenBuilder::default();
// Default xres and yres is 500
screen.xres = 500;
screen.yres = 500;
// Example:
//screen.xres = 723;
//screen.yres = 501;
```
