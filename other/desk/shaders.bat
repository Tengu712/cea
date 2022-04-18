@echo off
fxc /T vs_5_0 /E vs_main /Fo vshader.cso shaders.hlsl
fxc /T ps_5_0 /E ps_main /Fo pshader.cso shaders.hlsl
