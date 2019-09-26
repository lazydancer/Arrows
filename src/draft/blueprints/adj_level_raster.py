from functools import reduce

from draft.blueprints.raster import rasterize


def adj_levels_raster(blueprint_levels, connections):
    '''

    Attached locations from a grid

    inputs: [[Blueprint]], [[(Blueprint_1, Ouput_1), (Blueprint_2, Input_2)]

    '''
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
    A = defaults.get_split()
    A2 = defaults.get_split()
    B = defaults.get_cross()
    C = defaults.get_and()
    D = defaults.get_xor()
    E = defaults.get_and()


    blueprint_levels = [[A, A2], [B], [D, E]]
    connections = [[(A, 0), (D, 0)], 
                   [(A, 1), (B, 0)],
                   [(A2, 0), (B, 1)],
                   [(A2, 1), (E, 1)],
                   [(B, 0), (D, 1)],
                   [(B, 1), (E, 0)],
                ]

    return adj_levels_raster(blueprint_levels, connections)

