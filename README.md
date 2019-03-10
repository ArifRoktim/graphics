# w04\_curves

Assignment:

Add the following commands to the parser

* circle: adds a circle to the edge matrix - takes 4 parameters (cx, cy, cz, r)
* hermite: adds a hermite curve to the edge matrix - takes 8 parameters (x0, y0, x1, y1, rx0, ry0, rx1, ry1)
  * The curve is between points (x0, y0) and (x1, y1).
  * (rx0, ry0) and (rx1, ry1) are the rates of change at each endpoint
* bezier: adds a bezier curve to the edge matrix - takes 8 parameters (x0, y0, x1, y1, x2, y2, x3, y3)
  * This curve is drawn between (x0, y0) and (x3, y3)
  * (x1, y1) and (x2, y2) are the control points for the curve.
