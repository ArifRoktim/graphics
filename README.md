# w12\_final
Members: Arif Roktim  
Team Name: K***rust***y Krab

## Features to implement in descending order of importance

### Existing MDL Commands/features:
- [ ] Add `light` command; loop through all the lights
- [ ] Add `mesh` command
- [ ] Add `set`, `saveknobs`, and `tween` commands

### Time to up my compiler game:
- [ ] Add ability to reference variables (and do arithmetic on them). Useful for `vary`

Example use cases:
We have a script file with 100 frames. And the following vary commands:
```
vary biggenator 0 49 0 1
vary biggenator 50 99 1 0
```
If we want to halve the number of frames, we'd have to manually change `[0, 49]` and `[50, 99]` to `[0,24]` and `[25, 49]`.  
Very annoying. The following would be nice:
```
vary biggenator 0 (frames // 2 - 1) 0 1
vary biggenator (frames // 2) (frames - 1) 1 0
```
The `//` operator is integer division in this example. Also, all expressions would have to be in parentheses.  
For example, to rotate either less or more, based on number of frames:
```
vary spinMeRightRound 0 30 0 (frames)
```

### Shading:
- [ ] Gouraud shading
- [ ] Phong shading

### Additions/changes to MDL:
- [ ] Use `vary` to move lights
