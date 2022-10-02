# Big Dice Games Rust Raymarcher

This is a simple raymarcher, built as an exercise to teach myself Rust
and have fun making computer graphics.

## TODO

- Define scenes in JSON
- More geom primitives
  - triangle mesh
  - capped cone
  - teapot?
    - bezier patch
- boolean composition
  - union
  - difference
  - intersection
  - smooth addition
  - rounded corners
- transformations
  - scale(?)
  - rotation
  - combined rotation and translation as a matrix
- rewrite modules to have a <directory>.rs at toplevel
  - https://stackoverflow.com/questions/26435102/in-rust-what-is-the-purpose-of-a-mod-rs-file
  - geom
  - lights
  - colors
- documentation
- materials
  - plastic
    - kd
    - ks
    - ka
    - specular power
    - see https://en.wikipedia.org/wiki/Phong_reflection_model
  - metal (reflection)
    - roughness
  - wood (requires noise) 
    - see RMC p350
    - see also https://docs.rs/noise/latest/noise/struct.Cylinders.html
  - glass (refraction)
  - BRDF (there are a few databases of materials out there, would be cool to support them)
    - https://en.wikipedia.org/wiki/Bidirectional_reflectance_distribution_function
  - BSDF - surface descriptions including BRDF (reflection) and BTDF (transmission)
    - https://math.hws.edu/graphicsbook/c8/s2.html
- lights
  - cone
  - lights probably have position/direction
  - lights probably have a color
  - lights may have some sort of intensity/falloff
  - intensity for diffuse (should this be on lights?)
  - intensity for specular (should this be on lights?)
- simplex(?) noise https://docs.rs/noise/latest/noise/
- fog
- GLTF loading
  - e.g. Kenney.nl's car kit: https://www.kenney.nl/assets/car-kit
  - probably using https://crates.io/crates/gltf
- add colors from Crayola crayon boxes
  - wikipedia https://en.wikipedia.org/wiki/List_of_Crayola_crayon_colors
- material texture support (e.g. for posters, decals)
- "renderable object" that collects geometry, shader
- "scene" container for lights, renderable objects
  - loadable from JSON, above
- cmdline arg for JSON filename
- cmdline arg for resolution
- jenga blocks
  - 1.5cm x 2.5cm x 7.5cm
  - wood (above)
- soft shadows

## Done

- Actually raymarch
- camera
- colors
- image output
- geom primitives
  - z-plane
  - sphere
  - skydome
  - cube / box
  - torus
  - cylinder
    - infinite
    - capped
  - capsule    
- multi-threading
- normals from SDF for shading
- add colors from XKCD color survey https://blog.xkcd.com/2010/05/03/color-survey-results/
- add crayola colors
  - jenny's crayon collection http://www.jennyscrayoncollection.com/2017/10/complete-list-of-current-crayola-crayon.html
- lights
  - directional
  - ambient
  - point
- shadows
  - using ray marching proximity technique
- materials  
  - checkerboard (see RMC p345)
- sdf transformations
  - translation


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

[Update]
I implemented a function I have found super useful in previous
projects: map(v, in_min, in_max, out_min, out_max). In its simplest
form, it's just doing linear interpolation, and can be written on two
lines without being very clever.

I added an additional version, clamped_map, which clamps the input
value to lie between the two input references before doing the
interpolation.

To be extra fancy, I implemented it as a generic, which meant that it
would work with color values, or Vec3fs or whatever else I decide I
need it for. This took some amount of learning about how one
implements add, sub, and mul for user-defined types. It's not too
hard, and now I've got reference code to look at for the next time.

I also made a function (probably will rewrite it over time) that takes
an array of values, keyed by an input float, and interpolates an
output value smoothly lerping over the keys. I've used this before
with terrain rendering projects, where elevations below 0 are blue,
and then I have sand, grass, mountain rock, and snow/ice based on
elevation.

Using these, I rewrote the sky sphere, which now looks better.

I also added a distance fade to gray on the checkerboard shader, which
hides the nasty aliasing near the horizon.

I also hacked in some vaguely phong-like diffuse shading for the
spheres - it's not intended to be physically accurate, but it gives
some depth to the spheres that wasn't there before.

Also in the category of not-visible improvements, I added in
command-line argument processing, so now the user can specify the
number of threads to use, and the output file name. I kind of want to
write a Python script to run the program a number of times with
different numbers of threads to see if running exactly matching the
number of CPUs gives optimal performance.

![spheres_2022_09_24_a](https://user-images.githubusercontent.com/72338/192127540-92fe52ca-f3ae-4065-b2a3-2b0bef150101.png)


### September 25, 2022

Created a LightSource trait with (placeholder) implementations of cone
and point lights, a workable ambient light, and a first draft of a
directional light source.

Right now, the sphere shader has the diffuse lighting model built in,
and the ground has no lighting model, which makes things look a little
broken.

I also pulled in the XKCD color survey color names, so now
XKCD_BRIGHT_RED is a color, as is XKCD_BLUE_GREEN_1 and XKCD_POO.

And the big thing from this afternoon was checking for occlusion of
directional lights. The presumption is that the light is infinitely
far away (the sun is approximately infinitely far away), so I cast a
ray from the surface of the collided object in the direction of the
light (so, in the opposite direction of the incident light ray), and
if I collide with anything, I don't include that light's illumination.

I'm not satisfied with this, as it includes bumping the starting point
away from the object to keep it from immediately colliding with
itself. But sometimes, self-collisions are what you want. Maybe I just
make the SDF tolerance more easy to use as a constant, and bump the
start point further away than that.

![spheres_2022_09_25](https://user-images.githubusercontent.com/72338/192170112-368aa30c-a5a4-4c81-9ab9-14ca05209be0.png)

Also, movie on YouTube:

https://youtu.be/chuv2k4AczM

### October 1, 2022

The one visible thing that I've done today was to create a point light
source:



If you look at the checkins for today, though, I've touched just about
everything. Most of that is cleaning up my "use" blocks at the top of
many files. The Rust compiler is pretty good about giving you usable
suggestions, but the "use" suggestions it gives you can lead you to a
big list individual references, which isn't the best practice,
according to the book I'm currently reading.

Another thing I refactored was the way I'm handling scenes. For one
thing, now there's a Scene struct, which contains objects and lights
and a camera and a sky sphere (probably want to rewrite the sky sphere
thing, it's really just a shader - more thinking to be done).

So, what I wanted to do was something like "make a Scene with objects
from this vector of objects (the checkerboard floor) and this vector
of objects (some spheres) and some other vector of objects (a lamp
post, maybe). But to do that, I made a SceneBuilder, which is designed
to be mutable, and it was going to take a vector, but that wanted my
objects to be Copy and/or Clone-able, which they aren't, and stacking
up traits is tricky (they're already SDF trait objects, and adding
Clone to them seems like not what I want to do.

In the book I'm reading, I just read a section that talks about when
you want to use generics vs when you want to use traits. Maybe I
should reconsider some of the methods that currently take a Box<dyn
SDF + Sync>, maybe they could be generic, taking in a <S: SDF + Sync>
or some such. Generics are handled at compile-time, which could be a
benefit, but since the code I'm thinking about right now is building
the scene, that's not a huge win.

I should probably push this scene object through the various different
lighting and shading code that currently takes a list of lights and a
list of objects. That's what the scene is for, so that should tidy up
the code a bit.

With this refactoring, I went back to rebuild earlier test images -
Testing is Awesome! But then I panicked when some images turned out
flat shaded instead of diffuse shaded. I'm pretty sure what happened
along the way was that I had made earlier renders with a placeholder
diffuse implementation that never made it in to the Diffuse shader
object that I have now, so my building out of diffuse, ambient,
specular material implementations broke the old images. So I copied a
big chunk of code (sigh) from the specular material, pasted it in the
diffuse file, and deleted the glossy highlight. Bango, diffuse
support, including shadowing, which I think wasn't there before.



