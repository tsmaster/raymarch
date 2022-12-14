# Development Log

Being an in-order (recent at the bottom) listing of notes as I go.

## September 22, 2022

Set up project.

Made structs for Vec3f, Sphere and Z+Plane.


## September 23, 2022

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


## September 24, 2022

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


## September 25, 2022

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

## October 1, 2022

The one visible thing that I've done today was to create a point light
source:

![spheres_2022_10_01a](https://user-images.githubusercontent.com/72338/193434914-0860f4c1-6dc2-4e34-ad87-23bfa383f395.png)

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


[Update]
I added a cone light. I like it; it feels dramatic and a little spooky
as we enter October.

![spheres_2022_10_01b](https://user-images.githubusercontent.com/72338/193436243-d45b7c9b-54fe-4526-9a6a-982d28e5ca37.png)


## October 2, 2022

I added Constructive Solid Geometry (CSG) boolean operations, so now I
can take a disk, subtract two cylinders, and then subtract two spheres
and get a weird button shape:

![button_2022_10_02](https://user-images.githubusercontent.com/72338/193465771-88d71abd-1cc2-46d0-89b0-a0ba092bd9ca.png)


I also made a variant of my checkerboard shader to make a graph paper
shader:

![graph_paper_2022_10_02](https://user-images.githubusercontent.com/72338/193465781-44d5f440-33dd-4521-ad7e-75807281032b.png)


The super thin lines on the graph paper lead me to want to do
randomized supersampling to reduce some of the jaggies.


## October 5, 2022

I've been working on a GLTF model file loader. What I have so far uses
the easy-gltf crate to create a number of triangle "meshes" - I
hesitate to even call them that, because there's no real structure,
just a list of triangles, no vertices shared, no hint of what's
where. I also retrieve the base color for the associated material,
which is sufficient for the following render:

![car_2022_10_05](https://user-images.githubusercontent.com/72338/194094458-4d9fa4fa-09ee-45c8-aade-6071216cafe3.png)


I think that looks cool, but there are several pieces that remain:

 - _interpolated normals_ : one thing that makes this look like the
   flat shaded wonderland of Dire Straits' "Money for Nothing" video
   is the sharp polygon edges along the wheel fenders. Side note: I
   wonder if there are data files approximating the 3d characters from
   that video. The GLTF that I have contains vertex normals, so I
   should be able to interpolate those and provide smoother looking
   shading, at least, even if the polygons themselves remain faceted.

 - _physically based materials_ : it's a car, it's supposed to be
   shiny, and indeed, the easy-gltf material data talks about metallic
   roughness, which means I need to dig into what the implicit model
   is that I should be implementing.

 - _oof, performance_ : the elephant in the roadster is that a 240x240
   debug thumbnail took 178 seconds and change, which is maybe not so
   bad if one wants to be charitable and say that there's thousands of
   unstructured triangles, all just anywhere. But I have no patience
   for that; I demand something like a million pixels in a few
   seconds. (Side note: there's another gltf crate, just called
   "gltf", which I tried using, and I noticed two things - I couldn't
   find the base material color, and the rustc compile times were
   suddenly like 25 seconds, up from a couple of seconds. So,
   easy-gltf has fixed both.) Two avenues for improving performance
   come to mind:

    - _precomputing triangle values_: Inigo Quilez is an amazing
      resource in SDF math. On one of his (many, excellent) pages, he
      talks about math behind finding the distance to a triangle. I
      translated his code directly, without initially understanding
      it, and I attempted to debug my results. It turns out I had
      ported his code correctly, but I was rendering the body of the
      car rotated (y-up) and penetrating the floor, so I didn't
      recognize what I was looking at. I was expecting to see a single
      tire, but instead, got the rear of the body. Along the way, I
      saw Inigo's mention that a lot of the values in the SDF
      calculation could be computed once for the entire triangle,
      which should make things a little faster. Also, there's a square
      root, which maybe not as bad in 2022 as back in 1995, but he
      points out that the square root can be deferred if you're doing
      a lot of triangle distances, and done once at the end, which is
      exactly my case.

    - _bounding volume hierarchy_ : This one, I think, is going to be
      substantially more useful. As I've indicated above, I'm
      uncomfortable with the unstructured soup of triangles, and would
      prefer to put nearby triangles together. I wrote "connected
      triangles" at first, because you'd think that'd help, but a)
      maybe not so much, and b) I'm not getting any sort of connected
      mesh out of easy-gltf, and I don't want to stitch triangles
      together. Instead, I can group triangles into a tree of bounding
      volumes. A bounding box for the entire mesh would be an easy
      starting point, though it will require a little more complexity
      to my ray traversal - checking the bounding box before checking
      the contained triangle distances, comparing that bound to the
      running total to discard the contained triangles in one
      batch. When a single cull works, it will save me thousands of
      triangles' worth of calculation, which is good, but I'm afraid
      that I'll get into the triangles more than I want for a single
      bounding volume to suffice. So, I will make a tree, splitting a
      mesh up recursively, probably doing a binary split along the
      long dimension, continuing until I get down to a target volume
      (e.g. 1 unit cube) or down to a target number of triangles
      (maybe 100?). And, while I start with bounding boxes, I'm
      reminded of a diagram (in Newman & Sproull?) illustrating
      bounding volumes on a wagon wheel, which shows boxes, spheres,
      and cylinders, with varying degrees of tightness. I'm fortunate
      in that my car model has wheels, which prompts me to try boxes,
      spheres, and cylinders for pieces of my data, and keep the
      volume at each node of my tree that has the least volume, but
      contains all of my data. I anticipate that this should not be
      difficult, and hopefully, should give good performance results.


## October 8, 2022

Following up the above ideas, I've done the following:

 - precomputed mesh data and moved the square root out of the
   individual triangle computation.

 - broken down triangle meshes into smaller pieces, each with a
   bounding box.

Aaaand, the results are not super encouraging. A 400x400 render of the
Kenney.nl "future racer" GLTF model takes ~20 seconds to render. I
added profiling using coz:

https://github.com/plasma-umass/coz/tree/master/rust

cos run --- ./target/release/raymarch -r 400x400

and that gives me a hotspot list, telling me which functions are
called more than others. I rewrote my clamp function to be one line,
which knocked things down to around 19 seconds, still an improvement,
but not as much as I had hoped for.


## October 9, 2022

Spent a chunk of time thinking about using "Hierarchical Cone
Tracing", based on 
http://www.fulcrum-demo.org/wp-content/uploads/2012/04/Cone_Marching_Mandelbox_by_Seven_Fulcrum_LongVersion.pdf

There are several bits in that slide deck / presentation that make
sense, and a few claims that I'm not sure that I agree with. But I can
play around with it and see if I agree.

Two claims that I want to poke at:

1) Subdividing must be powers of 2 so that new rays are continuations
of old rays. I think that this is inaccurate in a few ways. If the new
resolution is an integer (erm, new = (old - 1) * k + 1, maybe?)
multiple of the old resolution, maybe things line up fine, and you
might as well do 2 as anything else, maybe. Also, maybe lining up
isn't as important as presented in the slide deck, so long as you use
the minimum of your "parent" rays to determine your initial depth.

2) The slide deck says that you can't use this to do reflection or
shadow rays. I think maybe you can, but you'd have to rearrange your
recursive evaluation, which is maybe OK. TBD.


Related to the cone marching idea, I've wanted to make orthographic
and isometric cameras, in part as an experiment, in part as an actual
feature (TODO render a Zaxxon scene, a Marble Madness scene...). In
the context of cone tracing, orthographic views don't have beams that
expand out like cones through each pixel, the beams are uniform
cylinders extending to infinity. Not a huge deal for ray marching:

![car_2022_10_09](https://user-images.githubusercontent.com/72338/194776553-11db8776-c3d4-49a9-8c10-37a19b1b2862.png)


But if/when I do the "cone marching", the cones will be defined by a
getWidthAtDistance(d) method, which will increase linearly with
distance: w = k * d for a perspective camera and will remain constant:
w = k for an orthographic camera.


An unrelated cool thing about cone marching is that it can be useful
when trying to do antialiasing in shader code. It's possible to
project pixel bounds forward into the scene at a distant point,
determine that the world bounds are large relative to
e.g. checkerboard values, and thus just return an average color.


Thinking about the GLTF car that I've been working on - the
performance remains not as good as I want, and one realization
occurred to me - I can still benefit from more structure, even more
than the BVH structure has been giving me.

When I import the car, I get 16 separate meshes (different materials,
separate wheels...), which I proceed to split based on the long
dimension (the car is about 4x as long as it is high, so I separate
the triangles into 4 buckets, based on where the centroid(?) of the
triangle lies in Y, relative to the bounding box of the whole
body. This provides some sorting benefit, and if a near triangle is
visited early in the scene traversal, it might prune later chunks of
the body BVH from being traversed.

Maybe.

Turns out, I store these children in increasing Y, so, nose to tail,
but there's nothing special about that. Also, these buckets have a lot
of overlap, which is not surprising for splitting up a mesh of
triangles - the seam is going to be jagged and gross.

So, hitting an early triangle might help prune later buckets, but
probably only if I'm looking at the car from the vicinity of the
nose. (I haven't.) If I'm looking from the side, I'll probably have to
visit all the buckets. If I'm looking from the back, and again, my
buckets are stored and visited from the nose first proceeding to the
back, this means that I'm going to visit everything in the whole mesh,
moving towards the camera (painter's algorithm).

A Spatial Partition (see
https://gameprogrammingpatterns.com/spatial-partition.html) might be
employed to query a scene to find things in a finite query volume. In
my case, I would be doing well to walk the scene in approximate
nearest to furthest distance to increase the likelihood that I'll find
a nearby hit, and then the hierarchy of the BVH can leverage that to
prune out complexity that I don't need.

I could add something like an octree or a binary space partition, each
of which would allow me to traverse the objects in order, front to
back. Sub-objects might still use BVHs to enclose the geometry to
provide for better pruning.

Even simpler than an octree might be a grid, choosing traversal order
based on a dot product with the view vector.

One thing where my brain is saying "sure, BSPs etc work with finite
geometry, but you've got an infinite ground plane, how will that
work?". Yes, good job, brain, you're right. So I don't stick infinite
stuff into my grid or my octree, but I stick infinite things in a
separate list. There won't be a lot of them, and being infinite, maybe
it makes sense to visit them first.


## October 10, 2022

Added a "capped cone" primitive (it's been sitting near the top of my
TODO list for a while) and an "infinite cone" primitive (it was nearby
on Inigo's SDF page).

![crayon_box_2022_10_10](https://user-images.githubusercontent.com/72338/194900083-76683435-8f05-4421-abea-0180c3f90806.png)

That is, of course, capped cones plus cylinders to get recognizeable
crayon shapes.
  

I've been thinking about rendering ortho ("plan") views of cars to
make 2d sprites, but what'd be really fancy is automatically rendering
the car and the shadow as separate objects. I think I can do that in
JSON by having the car body be a thing and a background as a separate
thing. I'd render the car body with shadows, without the background to
get the body, and then I'd render the background and the car body, but
the car body would be flagged as "shadows only".


## October 15, 2022

I rearranged the markdown for this project, moving this devlog into a
docs folder (overdue), as opposed to being in the toplevel README. I
also split up a bunch of other stuff that used to be in the README,
like my TODO list(s). I think it's a little too granular right now, so
I'm thinking I'm going to scale it back a tiny bit, and see how that
goes.

Last night, I saw the Pet Shop Boys and New Order perform at the
Climate Pledge Arena, and it was a great show. Seems like a bit of a
non-sequitur for this dev log, and it sort of is, but behind each of
the bands was a projection screen, and, indeed, PSB used a translucent
screen about 2/3 of the way forward - some of their stuff was just
Neil and Chris all the way downstage, with a mic (for Neil) and some
sort of keyboard / display (for Chris), and two lampposts (think of
Singin' in the Rain).

Anyway, one or two projection screens, an opportunity to add visuals
to each of their songs. To my mind, PSB did more interesting things
visually, while New Order's visuals weren't as impressive, relying on
presentation standards like laser projectors (several lights that do
that fan effect, cutting through smoke), smoke, spotlights (focused on
a rotating disco ball, not nearly as impressive as I expected.

But, what I'm getting to, was that there were a number of 80s computer
graphics visuals projected onto the screen(s).

- PSB had a bunch of wireframe people walking right to left across the
  stage, at one point, Neil walked from downstage center, to downstage
  right, with the wireframe people marching behind him, roughly
  synchronized. Seems simple, but it was effective. Later, there were
  a lot more stick figures, synchonized, superimposed, so the place
  where the feet landed had 4-5 different people, all whose feet were
  planted at the same place.

- there was some sort of a plasma fade from red to magenta to blue
  along the near screen, as Neil slowly walked from downstage right to
  downstage left.

- this probably was New Order, there was an effect where wireframe
  cubes (so many cubes) were superimposed on each other, translating
  from far away (isometric projection?) towards the
  origin. Periodically, the cubes would rotate around z (while
  translating? Or not?)

- I seem to recall this being a PSB thing, but I know New Order
  (also?) did this: a series of nested rectangles, reminding me of a
  hangar on the Death Star, a perspective illusion. In the middle of
  these rectangles was a red (plasma?) planet.

- moon zoom - New Order had a sequence where there was a picture of
  the moon, with a black cloud near the center, in the middle of the
  black cloud was a tiny little moon (maybe the black cloud was
  supposed to be interpreted as a break in clouds? and you're seeing
  the small moon through the clouds?) with an infinite zoom

- lots of imagery of cars, from a spaghetti highway flyover, with lots
  of headlights/taillights, to some footage from OutRun

- a wireframe torus mesh, as seen from the inside - New Order had a
  couple related sequences that were connected. One was a wireframe
  mesh of a torus, made up of square tiles, all subdivided diagonally,
  so it was a triangular mesh, where all the triangles were right
  equilateral 45-45-90 triangles. Easy. But then, a variation on this
  was to draw diagonal lines on this mesh (and not show the
  wireframe), to get a helical effect, like threads on a screw, but if
  the screw was the outside wall of a constantly turning tunnel. I
  think if you were following the inside of a torus, and maybe were
  barrel rolling the entire time as well, that'd be about the same
  thing.

- scanline - In "Opportunities", PSB had a bunch of green raster
  lines, which didn't always obviously form images. Probably, some of
  the sub-sequences were just abstract, probably some evoked
  waveforms. At some point, there was a face, and there were coins. 

- streetlamps - the streetlamps on stage also were echoed in a visual
  sequence, repeating off into the distance (not to infinity, that'd
  be out of control?). There were some moments within this where
  various lights would toggle on and off, in various patterns (mostly
  odd/even in a slow chase IIRC).


I mention all of these sequences, these images as inspirations for my
own creations. 
