# w09\_refactor

Assignment: Implement the following mdl commands:

* push
  * push a copy of the current top of the origins stack onto the origins stack
* pop
  * removes the top of the origins stack
* move/rotate/scale
  * create a translation/rotation/scale matrix and multiply the current top by it
  * do not try to use the optional arguments for these commands
* box/sphere/torus
  * add a box/sphere/torus to a temporary polygon matrix, multiply it by the current top and draw it to the screen
  * if a constants variable is present, use those for lighting, otherwise, use a default set.
  * ignore the optional variable at the end of the command.
* constants
  * you actually don't need to do anything for this command, the semantic analyzer will already create a symbol table entry for the reflective constants.
* line
  * add a line to a temporary edge matrix, multiply it by the current top and draw it to the screen
  * do not try to use the optional arguments for this command
* save
  * save the screen to the provided file name
* display
  * show the image
* You only need to modify one of the following files (c/python):
  * C
    * my\_main.c
      * look at print\_pcode.c, it is an ideal template to follow for my\_main.c
    * mdl.y: there is a comment at the very bottom that you will need to check.
  * Python
    * script.py

github clone links: https://github.com/mks66/mdl.git git@github.com:mks66/mdl.git

---

Complete all `TODO` items

TODO: use f64::mul\_add() when appropriate:

```
$ rg '(\*.*\+)|(\+.*\*)'
src/draw.rs
96:        let x_next = r * cos + cx;
97:        let y_next = r * sin + cy;
116:            + xs.m[0][1] * progress.powi(2)
117:            + xs.m[0][2] * progress
120:            + ys.m[0][1] * progress.powi(2)
121:            + ys.m[0][2] * progress
171:                r * cos_theta + cx,
172:                r * sin_theta * cos_phi + cy,
173:                r * sin_theta * sin_phi + cz,
191:            let p0 = lat * steps + longt;
193:            let p2 = (p1 + steps) % (steps * (steps - 1));
194:            let p3 = (p0 + steps) % (steps * (steps - 1));
232:                cos_phi * (minor_r * cos_theta + major_r) + cx,
233:                minor_r * sin_theta + cy,
234:                -1.0 * sin_phi * (minor_r * cos_theta + major_r) + cz,
259:            let p0 = lat * steps + longt;
261:            let p2 = (p1 + steps) % (steps * steps);
262:            let p3 = (p0 + steps) % (steps * steps);

src/matrix.rs
54:                    sum += self.raw()[self_row][self_col] * orig_other_row[self_row];
118:        let size: usize = (width + prec + 2) * (self.cols() + 2);

src/screen.rs
279:        let size: usize = PIXELS * 3 * 4 + YRES + 50;

src/vector.rs
31:        self.x * other.x + self.y * other.y + self.z * other.z
```
