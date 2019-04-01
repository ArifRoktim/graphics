# w06\_polygons

Assignment:

* Create new functions to add a polygon to a matrix, and go through the matrix 3 points at a time to draw triangles.
  * You should have a new triangle matrix that exists alongside the edge matrix.
    * The edge matrix should be used for the shapes that are exclusively 2d (lines, circles, splines).
    * The triangle matrix for our 3d shapes.
  * Anything aside from shape drawing that modifies/uses the edge matrix (apply, clear, display, save) should now modify/use the triangle matrix as well.
* Modify add box, add sphere and add torus to add triangles instead of points.
* Make sure the parser calls the draw\_polygons functions when needed instead of draw\_lines
* More to come...
* Vector math & Backface culling
  * Implement the following vector functions
    * Normalize a vector (provided as an array/list of 3 values)
    * Find the dot product of 2 vectors (provided as arrays/lists of 3 values)
    * Calculate the surface normal of a triangle in the polygon matrix (provided the polygon matrix and index.
    * Check out gmath.h/c or gmath.py for headers and comments.
  * Implement Backface culling.
