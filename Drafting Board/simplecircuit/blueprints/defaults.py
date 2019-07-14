from simplecircuit.common.block import Block

from simplecircuit.blueprints.blueprint import Blueprint
from simplecircuit.blueprints.raster import rasterize

def get_input_output():
    return Blueprint([0], [0], [[Block.wire_right]])

def get_and():
    return Blueprint([0, 2], [1], [
        [Block.wire_right, Block.negate_down , Block.space],
        [Block.space, Block.negate_right, Block.wire_right],
        [Block.wire_right , Block.negate_up, Block.space],
    ])

def get_or():
    return Blueprint([0, 2], [1], [
        [Block.wire_right , Block.wire_down, Block.space],
        [Block.space, Block.wire_right , Block.wire_right ],
        [Block.wire_right , Block.wire_up, Block.space],
    ])

def get_xor():
    return Blueprint([1, 3], [2], [
        [Block.space, Block.wire_right, Block.wire_down , Block.space, Block.space],
        [Block.wire_right, Block.split_down, Block.negate_right , Block.wire_down , Block.space],
        [Block.space, Block.negate_right , Block.split_down, Block.wire_right , Block.wire_right ],
        [Block.wire_right , Block.split_down, Block.negate_right , Block.wire_up, Block.space],
        [Block.space, Block.wire_right , Block.wire_up, Block.space, Block.space],
    ])

def get_split():
    return Blueprint([1], [0, 2], [
        [Block.space, Block.wire_right],
        [Block.wire_right, Block.split_down],
        [Block.space, Block.wire_right]
    ])

def get_cross():
    connections = set()

    split_a = get_split()

    split_b = get_split()
    
    xor_1 = get_xor()   
    connections.add((split_a.output(1), xor_1.input(0)))
    connections.add((split_b.output(0), xor_1.input(1)))

    split_final = get_split()
    connections.add((xor_1.output(0), split_final.input(0)))

    xor_a = get_xor()   
    connections.add((split_a.output(0), xor_a.input(0)))
    connections.add((split_final.output(0), xor_a.input(1)))

    xor_b = get_xor()   
    connections.add((split_b.output(1), xor_b.input(1)))
    connections.add((split_final.output(1), xor_b.input(0)))


    blueprints = [split_a, split_b, xor_1, split_final, xor_a, xor_b]

    locations = [
        (3, 0),
        (7, 0),
        (4, 2),
        (5, 7),
        (1, 9),
        (7, 9),
    ]

    board = rasterize(blueprints, connections, locations)

    return Blueprint([5, 8], [4, 9], board)

def get_half_adder():
    connections = []

    input_a = get_input_output()
    split_b = get_split()
    
    xor_1 = get_xor()   
    connections.append((input_a.output(0), xor_1.input(0)))
    connections.append((split_b.output(0), xor_1.input(1)))

    split_1 = get_split()
    connections.append((xor_1.output(0), split_1.input(0)))

    split_2  = get_split()
    connections.append((split_b.output(1), split_2.input(0)))

    xor_2 = get_xor()
    connections.append((split_1.output(1), xor_2.input(0)))
    connections.append((split_2.output(0), xor_2.input(1)))

    and_1 = get_and()
    connections.append((xor_2.output(0), and_1.input(0)))
    connections.append((split_2.output(1), and_1.input(1)))

    output_a = get_input_output()
    connections.append((split_1.output(0), output_a.input(0)))


    blueprints = [input_a, split_b, xor_1, split_1, split_2, xor_2, and_1, output_a]
    locations = [(1, 0),(3, 0), (0, 2), (1, 7), (5, 2), (2, 9), (4, 14), (1, 16)]

    board = rasterize(blueprints, connections, locations)

    return Blueprint([1, 4], [1, 5], board)

def get_full_adder():
    connections = []

    input_a = get_input_output()
    input_b = get_input_output()

    half_adder_1 = get_half_adder()
    connections.append((input_b.output(0), half_adder_1.input(1)))
    connections.append((input_a.output(0), half_adder_1.input(0)))
    
    carry_in = get_input_output()
    half_adder_2 = get_half_adder()
    connections.append((carry_in.output(0), half_adder_2.input(0)))
    connections.append((half_adder_1.output(0), half_adder_2.input(1)))
    
    or_1 = get_or()
    connections.append(((half_adder_2.output(1), or_1.input(0))))
    connections.append(((half_adder_1.output(1), or_1.input(1))))
    
    output_a = get_input_output()
    connections.append(((half_adder_2.output(0), output_a.input(0))))

    blueprints = [input_a, input_b, half_adder_1, carry_in, half_adder_2, or_1, output_a]
    locations = [(4,0), (7,0), (3, 3), (1, 0), (0, 20), (5, 37), (1, 39)]

    board = rasterize(blueprints, connections, locations)

    return Blueprint([1, 4, 7], [1, 6], board)


def get_four_adder():
    connections = []

    adder_1 = get_full_adder()
    
    adder_2 = get_full_adder()
    connections.append((adder_1.output(1), adder_2.input(0)))

    adder_3 = get_full_adder()
    connections.append((adder_2.output(1), adder_3.input(0)))

    adder_4 = get_full_adder()
    connections.append((adder_3.output(1), adder_4.input(0)))


    blueprints = [adder_1, adder_2, adder_3, adder_4]
    locations = [(0,1), (12, 1), (24, 1), (36, 1)]

    board = rasterize(blueprints, connections, locations)

    return Blueprint([], [], board) #TODO finish finding input and outputs
