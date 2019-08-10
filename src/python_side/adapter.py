import python_side.comm as comm

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


def send_board(board):
    adapter_board = comm.Engine_Board()
    for block in convert_board(board):
        adapter_board.add_block(*block)
    adapter_board.start()

def convert_board(board):
    for y, row in enumerate(board):
        for i, elem in enumerate(row):
            if elem == Block.space:
                continue
            yield (i, y, elem)