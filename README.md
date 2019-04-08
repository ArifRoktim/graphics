# w07\_cstack

Assignment:

Add/modify your current parser so it has the following behavior:

* push
  * push a copy of the current top of the coordinate system (cs) stack onto the cs stack
  * (a full copy, not just a reference to the current top)
* pop
  * removes the top of the cs stack
* move/rotate/scale
  * create a translation/rotation/scale matrix
  * multiply the current top of the cs stack by it
  * The ordering of multiplication is important here.
* box/sphere/torus
  * add a box/sphere/torus to a temporary polygon matrix
  * multiply it by the current top of the cs stack
  * draw it to the screen
  * clear the polygon matrix
* line/curve/circle
  * add a line to a temporary edge matrix
  * multiply it by the current top
  * draw it to the screen (note a line is not a solid, so avoid draw\_polygons)
* save
    * save the screen with the provided file name
* display
    * show the image
* Also note that the ident, apply and clear commands no longer have any use
