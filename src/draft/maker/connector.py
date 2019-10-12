import copy
from random import shuffle

from draft.model.block import Block


def connect(arrows, connections):
    for tries in range(100):
        shuffle(connections) # in-place shuffle
        result = _connections_try(arrows, connections)
        if result != None:
            return result
        print('Trying to find another path: try', tries)
    print('Could not find a solution stopped after 100 tries')

def _connections_try(arrows, connections):
    new_arrows = {}
    for a, b in connections:
        path = _connect_path(a, b, arrows)
        if path == None:
            return None
        new_arrows = {**new_arrows, **path}

    return {**arrows, **new_arrows}


def _connect_path(a, b, arrows):
    '''
    Joins two location together going from 'a' to 'b' on the 'board'
    '''
    # Simply board remove block and transform into an a 'free or wall'
    wall_board = [[False for _ in range(30)] for _ in range(30)]
    for x,y in arrows:
        wall_board[y][x] = True


    
    a = (a[0], a[1]) 
    #wall_board[a[0]][a[1]] = False
    wall_board[b[0]][b[1]] = False

    # Start a breath first search, finding the shortest path
    path = _breath_search(a, b, wall_board)

    if path == None:
        return None

    # Return a empty board with just the path
    new_arrows = {}
    for i in range(len(path)-1):
        y = path[i+1][1] - path[i][1]
        x = path[i+1][0] - path[i][0]


        if y == 1:
            new_arrows[(path[i][0],path[i][1])] = Block.wire_down
        elif y == -1:
            new_arrows[(path[i][0],path[i][1])] = Block.wire_up
        elif x == 1:
            new_arrows[(path[i][0],path[i][1])] = Block.wire_right
        elif x == -1:
            new_arrows[(path[i][0],path[i][1])] = Block.wire_left

    return new_arrows

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


