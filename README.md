# w09\_refactor

Assignment: Complete all `TODO` items

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
