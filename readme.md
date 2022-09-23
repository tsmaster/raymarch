# Big Dice Games Rust Raymarcher

This is a simple raymarcher, built as an exercise to teach myself Rust
and have fun making computer graphics.

## TODO

- Define scenes in JSON
- More geom primitives
  - cube
  - triangle mesh
  - cylinder
  - torus
  - teapot?
- boolean composition
  - union
  - difference
  - intersection
- transformations
  - scale
  - rotation
  - translation
- normals from SDF for shading
- preludes for submodules (e.g. geom)
- documentation
- materials
  - plastic
  - metal (reflection)
  - checkerboard (see RMC p345)
  - wood (requires noise) (see RMC p350)
  - glass (refraction)
- lights
- shadows
- simplex(?) noise
- fog

## Done

- Actually raymarch
- camera
- colors
- image output
- geom primitives
  - z-plane
  - sphere
  - skydome

## References

- some YouTube videos
  - Sebastian Lague: https://youtu.be/Cp5WWtMoeKg
  - SimonDev: https://youtu.be/BNZtUB7yhX4
  - Art of Code: https://youtu.be/PGtv-dBi2wE
  - Code Parade: https://youtu.be/svLzmFuSBhk
- Inigo Quilez's SDF page(s)
  - https://iquilezles.org/articles/distfunctions/
  - SDF of a line segment: https://youtu.be/PMltMdi1Wzg
  - SDF of a box: https://youtu.be/62-pRVZuS5c
- RenderMan Companion: https://archive.org/details/rendermancompani00upst/mode/2up
- Markdown cheat sheet: https://www.markdownguide.org/cheat-sheet/


## Dev Log

### September 22, 2022

Set up project.

Made structs for Vec3f, Sphere and Z+Plane.


### September 23, 2022

Implemented simple SDF ray march / "sphere cast" rendering.

I make a list of objects that implement the SDF (Signed Distance Field) trait,
along with a color (this should be a shader, eventually). I walk the scene,
using the distance to the closest object. If I get within a tolerance, I bail
out of the walk. If I never bail out, I return None.

The ground plane has a simple checkerboard shader.

If nothing gets hit, I shoot the ray at the sky sphere to get the sky color.





