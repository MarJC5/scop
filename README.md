<h1 align="center" style="text-align: center">
    <img alt="42Lausanne" title="42Lausanne" src="https://github.com/MarJC5/42/blob/main/42_logo.svg" width="140"> </br>
    Project n°16 - scop
    <h4 align="center" style="width: 50%; margin: 2rem auto; font-weight: normal; text-align: center">
        Take your first steps into the world of 3D on GPUs with OpenGL. The main concepts are approached during a small playful project.
    </h4>
</h1>

## Introduction

The goal of this project is to create a small 3D game engine using OpenGL. The engine will be able to load a 3D model in OBJ format, display it on the screen, and allow the user to interact with it using the keyboard and mouse.

### GPUs vs CPUs

A GPU is a processor specialized in parallel computing. It is composed of thousands of small cores that can perform calculations simultaneously. A CPU is a processor specialized in sequential computing. It is composed of a few large cores that can perform calculations one after the other.

### OpenGL

OpenGL is a cross-platform API for rendering 2D and 3D vector graphics. It is used in a wide range of fields, including CAD, virtual reality, scientific visualization, information visualization, flight simulation, and video games.

### GLFW

GLFW is a cross-platform library for creating windows with OpenGL contexts and managing input and events. It is commonly used as the windowing system in games with OpenGL.

## Documentations

- [OpenGL](https://www.opengl.org/)
- [GLFW](https://www.glfw.org/)
- [Learn OpenGL](https://learnopengl.com/)
- [Learn OpenGL - Camera](https://learnopengl.com/Getting-started/Camera)
- [Learn OpenGL - Model loading](https://learnopengl.com/Model-Loading/Model)
- [Learn OpenGL - Lighting](https://learnopengl.com/Lighting/Basic-Lighting)
- [Learn OpenGL - Materials](https://learnopengl.com/Lighting/Materials)

## Features

- [ ] 3D model loading in OBJ format
- [ ] 3D model display with textures and lighting (Phong shading)
- [ ] Camera movement (translation)
- [ ] Camera rotation (spherical coordinates)
- [ ] Camera zoom (field of view)
- [ ] Camera projection (perspective or orthographic)

## Installation

### Requirements

- [OpenGL](https://www.opengl.org/)
- [Rust](https://www.rust-lang.org/)
- [Make](https://www.gnu.org/software/make/)
- [GLFW](https://www.glfw.org/)

### Clone

Clone this repository to your local machine using `git clone https://github.com/MarJC5/scop.git` or download the zip file.

### Build

Build the project using `make`.

## Usage

Run the program using `make run` or from the `target/debug | target/release` folder using `./scop <model>`.
You can find some 3D models in the `resources/models` folder.

## Screenshots

Coming soon...

## Credits

- [rosdec](https://github.com/rosdec/rusted_opengl) for the OpenGL boilerplate in Rust (GLFW, GLEW, GLM)
