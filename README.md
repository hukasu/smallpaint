# Smallpaint
Smallpaint rewritten in Rust.  
TU Wien course [here](https://www.cg.tuwien.ac.at/courses/Rendering/VU.SS2019.html).

## Types
### Renderer
The `Renderer` manages the rendering of the scene.  

### Tracer
The `Tracer` calculates the bounces and returns the final color for a given pixel.  
There are 2 `Tracer`s available:  
| Name | Capabilities |
|---|---|
| SimpleTracer | <ul><li>Refractive Caustics</li></ul> |
| FresnelTracer | <ul><li>Refractive Caustics</li><li>Fresnel reflections</li></ul> |  

### Camera
The `Camera` generates rays for a given pixel in the "sensor".

There is 1 `Camera` available:  
| Name | Description |
|---|---|
| SimpleCamera | Pinhole camera |

### Terminator
The `Terminator` determinates if a ray should be ended or continue.  

There are 2 `Terminator`s available:  
| Name | Description |
|---|---|
| DepthTerminator | Stops the ray at a given `depth` |
| RussianRouletteTerminator | After the given `depth`, has a `probability` of stopping the ray |  

### Sampler
The `Sampler` generates vectors in a hemisphere to continue the path of a ray in a diffuse intersection.  

There are 2 `Sampler`s available:  
| Name | Description |
|---|---|
| RandomSampler | Generates vectors on a hemisphere randomly |
| HaltonSampler | Generates vectors on a hemisphere using numbers from the Halton Sequence.<br/>**Note**: This sampler is not recommended as it generates artifacts on the final image |  

### Scene
The `Scene` holds the geometries to be rendered.  

`Scene`s can have different types of storages. There is 1 `SceneObjectStorage` available:  
| Name | Description |
|---|---|
| Vec | Simple vector storage |  

`SceneObject`s represent the geometries that the scene contains. There are 3 `SceneObject`s available:
| Name | Description |
|---|---|
| Plane | An infinite plane |
| Sphere | An sphere |
| Cylinder | An cylinder. Can be `ThroughHole`, `SingleCap`, or `DoubleCap`.<br/>**Note**: `SceneObjects` with material `Refractive` can only be `DoubleCap`. |  

`SceneObject`s can be of 3 different `SceneObjectMaterial`. The material defines how the object interacts with the ray:
| Name | Description |
|---|---|
| Diffuse | Scatters lights in all directions |
| Specular | Reflacts light |
| Refractive | Transmits light |  

### Write
The `Write` writes the final output to a file.  

There is 1 `Writer`s available:  
| Name | Description |
|---|---|
| PPMWriter | Writes the output to a PPM file of type `P3` |