# raytracer

A parallel ray tracer written in Rust. Based on Peter Shirley's ["Ray Tracing in One
Weekend"](http://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf)

## Progress Pictures

Rendering settings:
- Resolution: 1920 x 1080
- 5000 samples per pixel
- Machine: GCP n1-highcpu-96 (96 vCPUs, 86.4 GB memory)

### Camera Lens Effects - 2019-07-05
Implemented camera aperture width and focal length effects. This is the end of the "Ray
Tracing in One Weekend" book. The render below includes a bunch of randomly generated
spheres and showcases the depth of field effect given by the camera lens parameters.
- Scene: [`scenes/spheres_3.json`](/scenes/spheres_3.json)
- Render time: 00:45:01

![](/img/spheres_3.png "Camera Lens Effects")

### Positionable Camera - 2019-07-03
An alternate angle of the previous scene using a custom camera position and FOV
- Scene: [`scenes/spheres_2_alt.json`](/scenes/spheres_2_alt.json)
- Render time: 00:01:48

![](/img/spheres_2_alt.png "Positionable Camera")

### Dielectric Materials - 2019-07-01
A simple scene with two metal spheres and a hollow glass sphere, showcasing the
reflective / refractive properties of dielectric materials
- Scene: [`scenes/spheres_2.json`](/scenes/spheres_2.json)
- Render time: 00:01:19

![](/img/spheres_2.png "Hollow Dielectric Sphere")

### Lambertian & Metal Materials - 2019-07-01
A simple scene with some metal and matte spheres, showcasing the scattering and
reflection properties of Lambertian and Metal materials.
- Scene: [`scenes/spheres.json`](/scenes/spheres.json)
- Render time: 00:01:22

![](/img/spheres.png "Lambertian & Metal spheres")

## Feature Roadmap

### Implemented
- [x] 3D Vector
- [x] Image generation
- [x] Spheres
- [x] Lambertian, Metal, and Dielectric surfaces
- [x] Multisampling
- [x] Parallel rendering
- [x] Loading scenes from JSON
- [x] Positionable Camera & custom FOV
- [x] Aperture & focal length customization

### Not Implemented
- [ ] Planes & Triangles
- [ ] BVH
- [ ] ADC
- [ ] Spectral rays
- [ ] Textures
- [ ] Lighting
- [ ] Shadow rays
- [ ] Volumes
- [ ] Phong Reflection (?)
- [ ] Photon Mapping (?)
