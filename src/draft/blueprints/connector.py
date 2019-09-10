import copy
from random import shuffle

from draft.common.block import Block

def connect(board, connections):

    temp_board = copy.copy(board)

    for tries in range(100):
        shuffle(connections) # in-place shuffle
        result = _connections_try(temp_board, connections)
        if result != None:
            return result
        print('Trying to find another path: try', tries)


    print('Could not find a solution stopped after 100 tries')

def _connections_try(board, connections):
    for a, b in connections:
        path = _connect_path(a, b, board)
        if path == None:
            return None
        board = _apply(path, 0, 0, board)

    return board


def _connect_path(a, b, board):
    '''
    Joins two location together going from 'a' to 'b' on the 'board'
    '''
    # Simply board remove block and transform into an a 'free or wall'
    wall_board = [list(map(lambda x: True if x != Block.space else False, row)) for row in board]
    
    a = (a[0], a[1] + 1) # Each start should be on the right
    #wall_board[a[0]][a[1]] = False
    wall_board[b[0]][b[1]] = False

    # Start a breath first search, finding the shortest path
    path = _breath_search(a, b, wall_board)

    if path == None:
        return None

    # Return a empty board with just the path
    result_board = [[None for elem in row] for row in board]

    for i in range(len(path)-1):
        y = path[i+1][0] - path[i][0]
        x = path[i+1][1] - path[i][1]

        if y == 1:
            result_board[path[i][0]][path[i][1]] = Block.wire_down
        elif y == -1:
            result_board[path[i][0]][path[i][1]] = Block.wire_up
        elif x == 1:
            result_board[path[i][0]][path[i][1]] = Block.wire_right
        elif x == -1:
            result_board[path[i][0]][path[i][1]] = Block.wire_left

    return result_board

def _breath_search(a, b, board):
    queue = [(a, [a])]
    visited = []

    while queue != []:
        loc, path = queue.pop(0)
        
        if loc == b:
            return path 

        for neighbour in [(loc[0] + 1, loc[1]), (loc[0], loc[1] + 1), (loc[0] - 1, loc[1]), (loc[0], loc[1] - 1)]:
            if( not ( (0 <= neighbour[0] < len(board)) and (0 <= neighbour[1] < len(board[0])) ) ):
                continue

            if board[neighbour[0]][neighbour[1]]:
                continue

            if neighbour in visited:
                continue

            visited.append(neighbour)
            queue.append((neighbour, path + [neighbour])) 

    return None

def _apply(block, at_x, at_y, prev_result):
    result = copy.deepcopy(prev_result)

    for j, y in enumerate(range(at_y, at_y + len(block))):
        for i, x in enumerate(range(at_x, at_x + len(block[0]))):
            if block[j][i] is None or block[j][i] == Block.space:
                continue
            result[y][x] = block[j][i]
    return result

# def test_connect():
#     board = [[Block.wire_right, Block.space, Block.space], 
#              [Block.space, Block.space, Block.space], 
#              [Block.split_left, Block.space, Block.wire_right]]
#     connect((0, 0), (2, 2), board)

# test_connect()

