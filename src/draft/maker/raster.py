import copy
from functools import reduce

from draft.model.block import Block
from draft.maker.connector import connect
from draft.maker.topological_sort import topological_sort


def raster(connections):
    adjancy_pairs = [(from_node[0], to_node[0]) for from_node, to_node in connections]
    levels = topological_sort(adjancy_pairs)

    return _place_on_grid(levels, connections)  
      

def _place_on_grid(levels, connections):
    '''
    Attached locations from a grid
    inputs: [[Blueprint]], [[(Blueprint_1, Ouput_1), (Blueprint_2, Input_2)]
    '''
    spacing_x, spacing_y = 2, 2
    locations = {}

    horizontal = []
    x = 0
    for level in levels:
        x += spacing_x
        horizontal.append(x)
        max_width = max([s.width() for s in level])
        x += max_width

    # vertical
    for i, level in enumerate(levels):
        y = 0
        for sec in level:
            y += spacing_y
            locations[sec] = (horizontal[i], y) 
            y += sec.height()

    sections = reduce(lambda x,y :x+y , levels) #flatten

    return _connect_rasterize(sections, connections, locations)


def _connect_rasterize(sections, connections, locations):
    result = {}

    for sec in sections:
        result = {**result, **_shift_section(sec, locations[sec])} # merge dictionaties


    connection_spots = _connection_spots(connections, sections, locations)
    
    #board = connect(board, list(connection_spots))

    return result 


def _shift_section(section, to_loc):
    result = {}
    for x,y in section.data:
            x_shift = x + to_loc[0]
            y_shift = y + to_loc[1]
            result[(x_shift, y_shift)] = section.data[(x,y)]
    return result

def _connection_spots(connections, sections, locations):
    result = []
    for conn in connections:
        a_bp, a_idx = conn[0]
        b_bp, b_idx = conn[1]

        a_loc = locations[a_bp]
        b_loc = locations[b_bp]

        a = (a_loc[0] + a_bp.outputs[a_idx], a_loc[1] + a_bp.width() - 1)
        b = (b_loc[0] + b_bp.inputs[b_idx], b_loc[1])

        result.append([a, b])

    return result