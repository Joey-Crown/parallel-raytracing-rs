This Rust program can be built and executed with the following Cargo command:

```cargo run --package parallel-raytracing-rs --bin parallel-raytracing-rs --release```

It will produce a file in the project root directory called "output.png".

You can change which scene is rendered by modifying the "main.rs" file in the "src" directory. The default scene is a Cornell Box with a single sphere in the middle. The random scene is a Cornell Box with 1000 randomly placed spheres. The random scene is commented out in the "main.rs" file. 
To render the random scene, uncomment the random scene code and comment out the default scene code.

Presently, the image should look like this:

![parallel raytracer output](https://i.imgur.com/aCFNVJ0.png)

A rough draft of a project report is in the root directory as "COP4520 Project Report.pdf".

Performance Notes:
Benchmarks were run on an intel i7-1165G7 CPU (2.8 Ghz, 4 cores, 8 threads) with 16 GB of RAM.

Single-threaded performance:

 Default scene (800x450, 200 samples per pixel, dev build): 441.68s

 Default scene (800x450, 200 samples per pixel, release build):  17.066s
 
 Random scene (900x600, 200 samples per pixel, release build):  793.037s (13m 13s)
 
Multi-threaded performance:

 Default scene (800x450, 200 samples per pixel, dev build):  125.47s

 Default scene (800x450, 200 samples per pixel, release build): 5.514s

 Random scene (900x600, 200 samples per pixel, release build): 233.793s (3m 53s)

