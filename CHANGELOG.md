## Change Log

### v0.12 
  - New dynamic bounding volume tree for use with spatial querying etc
  - Added collision primitives:
    * Particle (2D/3D)
    * Rectangle (2D)
    * Circle (2D)
    * ConvexPolygon (2D)
    * Cuboid (3D)
    * Sphere (3D)
    * ConvexPolyhedron (3D)
  - Added wrappers for collision primitives
    * Primitive2 (2D)
    * Primitive3 (3D)
  - Added support functions and bounding volume handling for all primitives
  - Added ray intersection testing for all primitives, both without and with object-to-world transformations
  - Added collision detection algorithms for broad phase collision detection:
    * Brute force (compare all bounding volume pairs)
    * Sweep and Prune (sorts volumes along a sweep axis, and compares bounding volumes where there is an overlap in that 
      axis)
  - Added algorithms that use GJK (Gilbert-Johnson-Keerthi) algorithm, and also EPA (Expanding Polytope Algorithm) for 
    additional contact data:
    * Convex primitive intersection testing
    * Convex primitive distance computation
    * Convex primitive time of impact intersection testing
    * Basic support for composite primitives
  - Reorganised code tree
  - Added unit tests and benchmarks 
