from functools import reduce

from draft.blueprints.raster import rasterize
from draft.blueprints.topological_sort import topological_sort


def raster_topo(connections):
    adjancy_pairs = [(from_node[0], to_node[0]) for from_node, to_node in connections]
    levels = topological_sort(adjancy_pairs)

    return place_on_grid(levels, connections)  
      


def place_on_grid(blueprint_levels, connections):
    '''
    Attached locations from a grid
    inputs: [[Blueprint]], [[(Blueprint_1, Ouput_1), (Blueprint_2, Input_2)]
    '''
    spacing_x, spacing_y = 2, 2
    locations = {}

    horizontal = []
    x = 0
    for level in blueprint_levels:
        x += spacing_x
        horizontal.append(x)
        max_width = max([s.width() for s in level])
        x += max_width

    # vertical
    for i, level in enumerate(blueprint_levels):
        y = 0
        for bp in level:
            y += spacing_y
            locations[bp] = (horizontal[i], y) 
            y += bp.height()

    blueprints = reduce(lambda x,y :x+y , blueprint_levels) #flatten

    return rasterize(blueprints, connections, locations)