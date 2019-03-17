# w05\_3d

Assignment:

* add the following commands to the parser
  * clear: clears the edge matrix of all points
    * c people: this is an incredibly simple operation that shouldn't involve futzing with any of the points in the edge matrix.
  * box: adds a rectangular prism (box) to the edge matrix - takes 6 parameters (x, y, z, width, height, depth)
  * sphere: adds a sphere to the edge matrix - takes 4 parameters (x, y, z, radius)
  * torus: adds a torus to the edge matrix - takes 5 parameters (x, y, z, radius1, radius2)
    * radius1 is the radius of the circle that makes up the torus
    * radius2 is the full radius of the torus (the translation factor). You can think of this as the distance from the center of the torus to the center of any circular slice of the torus.
* To future-proof your code, you should split sphere/torus creation into 2 parts:
  * Generating only the points on the surface of the shape.
  * Adding the points to an edge matrix so that they can be drawn.
    * Eventually, this part will be changed to handle solid shapes, but the points part will stay the same.
* You should actually add edges to draw the box.
* For the sphere and torus, **just add the points** for each point on the surface, and an edge from it to a point 1 pixel away to make it easier to see.
