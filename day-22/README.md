# Day 22

I originally solved this with a simple 3D array and brute force, but for the
refactor, I wanted to experiment with a Bounded Volume Hierarchy. This might
have been complete overkill though, given how long it has taken me to implement
it. Then again, now I have a working, if inefficient, BVH (no tree rotations yet,
so sorted input is problematic), which I can use for other projects.
