import copy

from draft.model.block import Block
from draft.blueprints.connector import connect

def rasterize(blueprints, connections, locations):
    '''
    Changes the blueprints to arrows
    TODO: remove the need for locations, to be discovered internally
    '''
    SIZE = (100, 100)

    board = _init_board(SIZE)

    board = _blueprints(board, blueprints, locations)

    connection_spots = _connection_spots(connections, blueprints, locations)
    board = connect(board, list(connection_spots))

    board = _trim_board(board)

    return board

def _init_board(size):
    return [[Block.space for _ in range(size[0])] for _ in range(size[1])]

def _blueprints(board, blueprints, locations):
    for i in range(len(blueprints)):
        board = _apply(blueprints[i].data, locations[i][1], locations[i][0], board)

    return board

def _connection_spots(connections, blueprints, locations):
    for conn in connections:
        a_bp, a_idx = conn[0]
        b_bp, b_idx = conn[1]

        a_loc = locations[blueprints.index(a_bp)]
        b_loc = locations[blueprints.index(b_bp)]

        a_width = len(a_bp.data[0]) -1

        a = (a_loc[0] + a_bp.outputs[a_idx], a_loc[1] +  a_width)
        b = (b_loc[0] + b_bp.inputs[b_idx], b_loc[1])

        yield [a, b]

def _trim_board(board):
    # Rows: start at the bottom and work till a not space is seen
    for i in range(len(board)-1, -1, -1):
        all_space = all(Block.space == x for x in board[i])
        if not all_space:
            board = board[:i+1]
            break

    # Columns: start at the right and work till a not space is seen
    for i in range(len(board[0])-1, -1, -1):
        all_space = all(Block.space == x[i] for x in board)
        if not all_space:
            board = [x[:i+1] for x in board]
            break

    return board


def _apply(block, at_x, at_y, prev_result):
    result = copy.deepcopy(prev_result)

    for j, y in enumerate(range(at_y, at_y + len(block))):
        for i, x in enumerate(range(at_x, at_x + len(block[0]))):
            if block[j][i] is None or block[j][i] == Block.space:
                continue
            result[y][x] = block[j][i]
    return result