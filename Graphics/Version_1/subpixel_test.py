#!/usr/bin/env python

import math
import cairo

WIDTH, HEIGHT = 256, 256

surface = cairo.ImageSurface(cairo.FORMAT_ARGB32, WIDTH, HEIGHT)
ctx = cairo.Context(surface)
ctx.set_antialias(cairo.ANTIALIAS_SUBPIXEL)

ctx.scale(WIDTH, HEIGHT)  # Normalizing the canvas

ctx.rectangle(0, 0, WIDTH, HEIGHT)
ctx.set_source_rgb(0, 0, 0)
ctx.fill()

ctx.arc(0.4, 0.4, 0.2, 0, 2*math.pi)
ctx.set_source_rgba(1,1,1)
ctx.fill()

print(ctx.get_antialias())

surface.write_to_png("example.png")  # Output to PNG
