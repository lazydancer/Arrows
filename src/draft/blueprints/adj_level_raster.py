from functools import reduce

from draft.blueprints.raster import rasterize
from draft.blueprints.topological_sort import topological_sort
import draft.blueprints.defaults as defaults


def raster_topo(connections):
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