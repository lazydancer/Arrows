'''
The purpose is to take in an adjacentcy list of blueprints and output the rastered image

[   
    [A],  
    [B, C],
    [D]
]


    A   B   D
        C

'''

from functools import reduce # Python3

from draft.blueprints.raster import rasterize


def adj_levels_raster(blueprint_levels, connections):
    spacing_x, spacing_y = 2, 2
    locations = []

    # Spacing from the input to the first
    x = spacing_x

    for level in blueprint_levels:
        y = spacing_y

        for bp in level:
            locations.append((y, x)) 

            y += len(bp.data)
            y += spacing_y

        
        max_width = max([len(x.data[0]) for x in level])

        x += max_width
        x += spacing_x 

    blueprints = reduce(lambda x,y :x+y , blueprint_levels)

    return rasterize(blueprints, connections, locations)

import draft.blueprints.defaults as defaults

def test_adj_levels_raster():
    A = defaults.get_and()
    B = defaults.get_and()
    C = defaults.get_and()
    D = defaults.get_and()

    blueprint_levels = [[A], [B, C], [D]]

    return adj_levels_raster(blueprint_levels, [])

