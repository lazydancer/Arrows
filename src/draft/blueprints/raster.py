import copy

from draft.model.block import Block
from draft.blueprints.connector import connect

def rasterize(blueprints, connections, locations):
    result = {}

    for bp in blueprints:
        result = {**result, **shift_section(bp, locations[bp])} # merge dictionaties


    connection_spots = _connection_spots(connections, blueprints, locations)
    
    #board = connect(board, list(connection_spots))

    return result 


def shift_section(section, to_loc):
    result = {}
    for x,y in section.data:
            x_shift = x + to_loc[0]
            y_shift = y + to_loc[1]
            result[(x_shift, y_shift)] = section.data[(x,y)]
    return result

def _connection_spots(connections, blueprints, locations):
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