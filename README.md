# w11\_mdl\_animation

IMPORTANT TODO(S):
* Remove all old frames from the directory when creating animations

New MDL commands to implement:
* frames
  * set the number of frames
* basename
  * sets the basename for animation file saving
* vary
  * vary the values for a knob between 2 values in a set range of frames

Animation Features:

The key animation commands are frames, basename and vary. You should proceed with animation code in 3 steps:
* Go through the operations list and look for any of the three animation commands
  * Set frames and basename
  * Handle errors as needed
* Go through the operations list a second time and look for the vary command
  * Populate a table that has an entry for each frame, and in each frame it has a value for each knob
    * When completed, the table should contain the correctly set values for each knob (perform the varying calculation)
    * In c, there is a struct vary\_node defined in parser.h
    * In python, you could use a dictionary/list combination
    * Handle errors as needed
* Perform the normal drawing steps, with the following additions if animation code is present
  * Look at the table of knob values (set in the second step) and set each knob in the symbol table to the appropriate value.
  * Run the normal commands
  * At the end of the loop, save the current screen to a file, the file should have the basename followed by a number, so that animate will work correctly.
    * I suggest you put all the animation frames in a subdirectory, so just append a directory name to the basename when saving files
  * When you are done with each frame loop, don't forget to reset the screen, origin stack and any other pieces of data that are specific to a given frame

Once you have all the files created, you can generate the animation using imagemagick's animate and convert commands:
* animate
  * Will display multiple single image files in succession as a single animation, with a default frame rate of 100 frames per second, by using the -delay option, you can change the fps ( -delay x will set the frame rate to 100/x fps )
  * $ animate -delay 1.7 animations/orb\*
* Convert can, like animate, take a number of frames and animate them, but instead of displaying the animation, it will combine them into a single animated gif file. Note that the only image format that can use animation is gif.
  * $ convert -delay 10 animations/orb* orb.gif will create a single animated gif called orb.gif
  * In python and c, I've included a make\_animation function in display.c/py that will generate the animation for you.
