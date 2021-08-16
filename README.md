# Rust ray tracer

I'm learning [Rust](https://www.rust-lang.org/) and decided to tackle the classic [Ray Tracing in One Weekend](https://raytracing.github.io/) project by Peter Shirley.  All the algorithms and general code structure are from that book, just translated the examples from C++ into Rust.  I haven't written a ray tracer before, but I hope to reimplement it in a few other languages after Rust, like [Zig](https://ziglang.org/) and the language I know best, JavaScript, just for a performance comparison.

It supports command-line and WebAssembly.

## Blog posts

 - [Shaking off the Rust 1: Ray tracing in One Weekend](https://clayto.com/2021/02/shaking-off-the-rust-1-ray-tracing-in-one-weekend/)

## Example renders

Here are some renders, some are to document subtle bugs, and some are just to celebrate first successful renders with new features!


![](cli/renders/raytrace-1616044535.567072104s.final-book-image-render-15hours.jpg)
![](./cli/renders/raytrace-1616444535.747858648s.png)
![first hires reflective](cli/renders/raytrace-1615001082.58049436s.first-hires-reflective.jpg)
![fixed metal reflectance wooooo](cli/renders/raytrace-1614999320.758953735s.fixed-metal-reflectance-wooooo.jpg)
![first render with materials and attenuation](cli/renders/raytrace-1614997695.127485347s.first-render-with-materials-and-attenuation.jpg)
![first render with materials](cli/renders/raytrace-1614997198.513695391s.first-render-with-materials.jpg)
![first-hires-render-with-clean-shadows](cli/renders/raytrace-1614011092.297087907s.first-hires-render-with-clean-shadows.png)
![5-raycolor-multiplier](cli/renders/raytrace-1614010469.65369906s.found-missing-0.5-raycolor-multiplier.png)
![shadow-acnestriations-caused-by-shadow-acne](cli/renders/raytrace-1614010364.320713325s.shadow-acnestriations-caused-by-shadow-acne.png)
![bad-fix-for-shadow-acne](cli/renders/raytrace-1613672941.092348252s.bad-fix-for-shadow-acne.jpg)
![gamma-correction](cli/renders/raytrace-1613672318.912683239s.gamma-correction.jpg)
![24m-striations](cli/renders/raytrace-1613668468.833009814s.24m-striations.jpg)
![hires-bounces-with-striations](cli/renders/raytrace-1613664127.18419605s.hires-bounces-with-striations.jpg)
![first-with-bounces-yaaaaaay](cli/renders/raytrace-1613660537.637976283s.first-with-bounces-yaaaaaay.jpg)
![now-with-AA](cli/renders/raytrace-1613423760.281383766s.now-with-AA.png)
![oh-just-flubbed-normal-colors](cli/renders/raytrace-1613236717.883664395s.oh-just-flubbed-normal-colors.png)
![oh-just-noramsl](cli/renders/raytrace-1613156168.617724519s.oh-just-noramsl.png)
![ahhhh-the-blood-sun](cli/renders/raytrace-1613154936.045811774s.ahhhh-the-blood-sun.png)
![ahh-a-nice-sunny-day](cli/renders/raytrace-1613153282.619191059s.ahh-a-nice-sunny-day.png)
![first-render](cli/renders/raytrace-1612814365.15628162s.first-render.png)
