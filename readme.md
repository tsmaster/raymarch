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
    - kd
    - ks
    - ka
  - metal (reflection)
  - checkerboard (see RMC p345)
  - wood (requires noise) 
    - see RMC p350
    - see also https://docs.rs/noise/latest/noise/struct.Cylinders.html
  - glass (refraction)
- lights
- shadows
- simplex(?) noise https://docs.rs/noise/latest/noise/
- fog
- GLTF loading
  - e.g. Kenney.nl's car kit: https://www.kenney.nl/assets/car-kit
  - probably using https://crates.io/crates/gltf
- add colors from XKCD color survey https://blog.xkcd.com/2010/05/03/color-survey-results/
- material texture support (e.g. for posters, decals)

## Done

- Actually raymarch
- camera
- colors
- image output
- geom primitives
  - z-plane
  - sphere
  - skydome
- multi-threading

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

I make a list of objects that implement the SDF (Signed Distance
Field) trait, along with a color (this should be a shader,
eventually). I walk the scene, using the distance to the closest
object. If I get within a tolerance, I bail out of the walk. If I
never bail out, I return None.

The ground plane has a simple checkerboard shader.

If nothing gets hit, I shoot the ray at the sky sphere to get the sky color.

![spheres_2022_09_23](https://user-images.githubusercontent.com/72338/192061994-d26847e0-4dfa-4fe7-9975-a72ce24445ae.png)

There's a green rim noticeable on the blue spheres. This is likely due
to an error in running out of iterations before getting within the
sphere tolerance, we keep taking smaller and smaller steps, but we
don't register a collision.  This can be fixed by increasing the
number of steps and/or loosening the tolerance.

There's also green on the horizon, which is a side effect of the
sloppy WIP sky sphere, combined with only rendering out a distance of
1000.0 units. Updating the sky sphere and/or increasing the render
distance would provide more plane out near the horizon.  Pushing the
far boundary would have decreasing returns, while making the sky
sphere more able to hide this sort of artifact would probably be more
effective.


### September 24, 2022

No visual changes, but got multithreading working. I've heard folks
say that once it compiles, you can trust it, but I'm still struggling
to get stuff to compile the first time I write it.