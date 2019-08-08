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


block_groups = {
    'wires': [Block.wire_left, Block.wire_up, Block.wire_right, Block.wire_down],
    'negates': [Block.negate_left, Block.negate_up, Block.negate_right, Block.negate_down],
    'splits': [Block.split_left, Block.split_up, Block.split_right, Block.split_down],
    'space': [Block.space],
}
