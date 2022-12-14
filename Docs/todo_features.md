# TODO (Features)


## TODO
- Define scenes in JSON
- More geom primitives
  - teapot?
    - bezier patch
    - http://www.holmes3d.net/graphics/teapot/
- boolean composition
  - smooth addition ("Smooth Union, Subtraction and Intersection - bound, bound, bound")
  - rounded corners https://iquilezles.org/articles/distfunctions/ ("Rounding - exact")
- transformations
  - scale(?)
  - rotation
  - combined rotation and translation as a matrix
- rewrite modules to have a <directory>.rs at toplevel
  - https://stackoverflow.com/questions/26435102/in-rust-what-is-the-purpose-of-a-mod-rs-file
  - geom
  - lights
  - colors
  - shaders
- documentation
- materials
  - matte (see RMC p 334)
  - plastic (see RMC p 336)
    - kd
    - ks
    - ka
    - specular power
    - see https://en.wikipedia.org/wiki/Phong_reflection_model
  - metal (reflection) (see RMC p 336)
    - roughness
  - wood (requires noise) 
    - see RMC p350
    - see also https://docs.rs/noise/latest/noise/struct.Cylinders.html
  - glass
    - has reflective surface
    - has refraction internal
    - may have frosted surface
    - may have colored "participating media"
    - what if: sparkles
  - BRDF (there are a few databases of materials out there, would be cool to support them)
    - https://en.wikipedia.org/wiki/Bidirectional_reflectance_distribution_function
    - https://www.scratchapixel.com/lessons/3d-basic-rendering/phong-shader-BRDF?url=3d-basic-rendering/phong-shader-BRDF 
  - BSDF - surface descriptions including BRDF (reflection) and BTDF (transmission)
    - https://math.hws.edu/graphicsbook/c8/s2.html
  - spectrum sparkle
    - no joke, as I've been thinking about materials, I saw a car
      getting on 522 near my house that was mostly white, but it had
      random sparkles (maybe one visible sparkle point per inch?)
      which reminded me of an effect from some stickers in the 80s, or
      even looking at a CD or DVD, where there were colors that moved
      based on the viewing angle.
- simplex(?) noise https://docs.rs/noise/latest/noise/
- fog
- GLTF loading
  - handle triangle vertex normals
- add colors from Crayola crayon boxes
  - wikipedia https://en.wikipedia.org/wiki/List_of_Crayola_crayon_colors
- material texture support (e.g. for posters, decals)
- "renderable object" that collects geometry, shader
- "scene" container for lights, renderable objects
  - loadable from JSON, above
- cmdline arg for JSON filename
- other building blocks, including 2x4x1 blocks with semi-circle cuts
  for arches, bridges
- soft shadows
- revisit sky
  - just a shader?
  - implement star map
  - maybe a city skyline
  - trees
  - https://www.scratchapixel.com/lessons/procedural-generation-virtual-worlds/simulating-sky
- support RGBA output
  - useful when rendering without a sky
  - useful when rendering sprites for 2d games
- fancy selector shaders
  - select based on 3 x y z hat vectors dotted with normal
  - select based on 6 x y z hat vectors dotted with normal
  - select based on 1 vector (e.g. grass on top, dirt elsewhere)
  - select based on SDF; positive is mat1, negative is mat2
    - allows me to shade the inside of dice pips based on using similar "masks" to the carving out geom
- geometry selector
  - e.g. LOD based on distance
- repeat modifiers
  - xyz
  - xy
- randomized offset based on position
  - for e.g. forest
- marching cubes?
- supersampling
  - grid based
  - randomized
- fractal landscape https://iquilezles.org/articles/fbmsdf/
  - https://www.shadertoy.com/view/Ws3XWl
- interpolate normals over triangle mesh
  - https://codeplea.com/triangular-interpolation
- motion blur
  - time-extended transformations
- benchmark tests
  - https://doc.rust-lang.org/1.16.0/book/benchmark-tests.html
  - https://github.com/elalfer/rust-performance-timing
- cone tracing for hierarchical performance improvement
- cone/beam bounds turned into world bounds at the point of collision,
  for more informed shading.
  - e.g. a checkerboard shader fading to an average color to reduce
    high frequency aliasing
- tagging of "layers" to enable rendering car counter sprites
  - render the body first, with no ground plane, with alpha skybox
  - render the ground underneath to get (soft?) shadows, but with the
    car geometry flagged as only visible to shadows
  - save both as RGBA images
