from functools import reduce

from draft.blueprints.raster import rasterize
from draft.blueprints.topological_sort import topological_sort
import draft.blueprints.defaults as defaults


def topo_test():
    A = defaults.get_split()
    A2 = defaults.get_split()
    B = defaults.get_cross()
    C = defaults.get_and()
    D = defaults.get_xor()
    E = defaults.get_and()


    connections = [[(A, 0), (D, 0)], 
                   [(A, 1), (B, 0)],
                   [(A2, 0), (B, 1)],
                   [(A2, 1), (E, 1)],
                   [(B, 0), (D, 1)],
                   [(B, 1), (E, 0)],
    ]

    adjancy_pairs = [(from_node[0], to_node[0]) for from_node, to_node in connections]
    levels = topological_sort(adjancy_pairs)

    return adj_levels_raster(levels, connections)  
      


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