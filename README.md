# w08\_solids

Assignment:

Add scanline conversion and z-buffering to your graphics engine.
* Parser Note:
  * In the previous assignment, I noted that the clear command was no longer needed.
    * Now, clear will clear the screen and zbuffer.
  * I have added this command to the provided source code, but everyone should implement it.
* Scanline conversion
  * Create a new function that handles the scanline conversion.
  * Call this in your draw\_polygons function.
  * Make sure that you change color values for each triangle.
* z-buffering
  * The z-buffer should only be modified in your plot function, or when clear\_zbuffer is called.
  * You will need to calculate z values in both scanline\_convert and draw\_line.
    * Your z values are not limited to the integers.

GitHub repository: https://github.com/mks66/solids.git git@github.com:mks66/solids.git
