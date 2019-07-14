def solve(board, state):
    new_state = []
    for row in state:
        new_row = []
        for item in row:
            new_row.append(item)
        new_state.append(new_row)

    for y in range(len(board)):
        for x in range(len(board[0])):
            surround = get_surround(state, y, x)
            if board[y][x] == None:
                new_state[y][x] = (False, False, False, False)
            new_state[y][x] = solver[board[y][x]](surround)

    return new_state

def get_surround(state, y, x):
    border_y = len(state)
    border_x = len(state[0])

    left = state[y][x-1][2] if x-1 >= 0 else None
    right = state[y][x+1][0] if x+1 < border_x else None
    up = state[y-1][x][3] if y-1 >= 0 else None
    down = state[y+1][x][1] if y+1 < border_y else None

    return (left, up, right, down)


solver = {
    0: lambda x:wire(0, x),
    1: lambda x:wire(1, x),
    2: lambda x:wire(2, x),
    3: lambda x:wire(3, x),
    4: lambda x:negate(0, x),
    5: lambda x:negate(1, x),
    6: lambda x:negate(2, x),
    7: lambda x:negate(3, x),
    8: lambda x:splitter(0, x), 
    9: lambda x:splitter(1, x),
    10:lambda x:splitter(2, x),
    11:lambda x:splitter(3, x),
    12:lambda _:(False, False, False, False),
}

def leftShift(tup, n):
    try:
        n = n % len(tup)
    except ZeroDivisionError:
        return tuple()
    return tup[n:] + tup[0:n]

def wire(facing, dirs):
    # default arrow '←'
    updated_dir = leftShift(dirs,  facing)
    if updated_dir[1] or updated_dir[2] or updated_dir[3]:
        return leftShift((True, False, False, False), -1 * facing)
    return (False, False, False, False)

def negate(facing, dirs):
    # default arrow '↞'
    updated_dir = leftShift(dirs, facing)
    if updated_dir[1] or updated_dir[2] or updated_dir[3]:
        return (False, False, False, False)
    return leftShift((True, False, False, False), -1 * facing)
    
def splitter(facing, dirs):
    # default arrow '↔'
    updated_dir = leftShift(dirs, facing)
    if updated_dir[1] or updated_dir[3]:
        return leftShift((True, False, True, False), -1 * facing)
    return (False, False, False, False)