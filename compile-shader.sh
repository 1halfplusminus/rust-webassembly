#!/usr/bin/env bash

glslc ./src/shaders/part-1.vert -o ./src/shaders/part-1.vert.spv
glslc ./src/shaders/part-1.frag -o ./src/shaders/part-1.frag.spv

glslc ./src/shaders/part-2.vert -o ./src/shaders/part-2.vert.spv
glslc ./src/shaders/part-2.frag -o ./src/shaders/part-2.frag.spv