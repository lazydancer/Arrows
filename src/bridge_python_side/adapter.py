import bridge_python_side.comm as comm

# This is from draft...
# there should be a way to keep these both insync
class Block:
    wire_left = 0
    wire_up = 1
    wire_right = 2
    wire_down = 3
    negate_left = 4
    negate_up = 5
    negate_right = 6
    negate_down = 7
    split_left = 8
    split_up = 9
    split_right = 10
    split_down = 11
    space = 12


def send_sparse(arrows):
    '''
    arrows is a dictionary, (x, y) => block
    '''
    adapter_board = comm.Engine_Board()
    
    for x, y in arrows:
        adapter_board.add_block(x, y, arrows[(x, y)])
    
    adapter_board.start()
    