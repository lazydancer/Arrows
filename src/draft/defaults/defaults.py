from draft.model.block import Block
from draft.model.section import Section_Section, Section_Data, Section_Flex

def get_input_output():
    return Section_Data([0], [0], [[Block.wire_right]])

def get_and():
    return Section_Data([0, 2], [1], {
        (0,0): Block.wire_right,
        (1,0): Block.negate_down,
        (1,1): Block.negate_right, 
        (2,1): Block.wire_right,
        (0,2): Block.wire_right,
        (1,2): Block.negate_up,
    })

def get_or():
    return Section_Data([0, 2], [1],{
        (0,0): Block.wire_right,
        (1,0): Block.wire_down,
        (1,1): Block.wire_right,
        (2,1): Block.wire_right,
        (0,2): Block.wire_right,
        (1,2): Block.wire_up,
    })

def get_xor():
    return Section_Data([1, 3], [2], {
        (1,0): Block.wire_right,
        (2,0): Block.wire_down,
        (0,1): Block.wire_right,
        (1,1): Block.split_down,
        (2,1): Block.negate_right,
        (3,1): Block.wire_down,
        (1,2): Block.negate_right,
        (2,2): Block.split_down,
        (3,2): Block.wire_right,
        (4,2): Block.wire_right,
        (0,3): Block.wire_right,
        (1,3): Block.split_down,
        (2,3): Block.negate_right,
        (3,3): Block.wire_up,
        (1,4): Block.wire_right,
        (2,4): Block.wire_up,
    })

def get_split():
    return Section_Data([1], [0, 2], {
        (1,0): Block.wire_right,
        (2,0): Block.wire_right,
        (0,1): Block.wire_right,
        (1,1): Block.split_down,
        (1,2): Block.wire_right,
        (2,2): Block.wire_right,
    })

def get_cross():
    split_a = get_split()
    split_b = get_split()
    xor_1 = get_xor()
    split_final = get_split()
    xor_a = get_xor()
    xor_b = get_xor()


    section = Section_Flex(
        [split_a.input(0), split_b.input(0)],
        [xor_a.output(0), xor_b.output(0)],
        [
            [split_a.output(1), xor_1.input(0)],
            [split_b.output(0), xor_1.input(1)],
            [xor_1.output(0), split_final.input(0)],
            [split_a.output(0), xor_a.input(0)],
            [split_final.output(0), xor_a.input(1)],
            [split_b.output(1), xor_b.input(1)],
            [split_final.output(1), xor_b.input(0)],
        ]
    )

    return section


def four_input_and():

    data = {
        (0,0): get_and(),
        (0,3): get_and(),
        (3,2): get_and(),
    }

    return Section_Section(data)


def half_adder():
    split_b = get_split()
    xor_1 = get_xor()   
    split_1 = get_split()
    split_2  = get_split()
    xor_2 = get_xor()
    and_1 = get_and()

    inputs = [xor_1.input(0), split_b.input(0)]
    outputs = [split_1.output(0)]
    connections = [
        [split_b.output(0), xor_1.input(1)],
        [xor_1.output(0), split_1.input(0)],
        [split_b.output(1), split_2.input(0)],
        [split_1.output(1), xor_2.input(0)],
        [split_2.output(0), xor_2.input(1)],
        [xor_2.output(0), and_1.input(0)],     
        [split_2.output(1), and_1.input(1)],
    ]


    {
        (3, 0)
    }


    return Section_Flex(inputs, outputs, connections)

# def get_half_adder():
#     connections = []

#     input_a = get_input_output()
#     split_b = get_split()
    
#     xor_1 = get_xor()   
#     connections.append((input_a.output(0), xor_1.input(0)))
#     connections.append((split_b.output(0), xor_1.input(1)))

#     split_1 = get_split()
#     connections.append((xor_1.output(0), split_1.input(0)))

#     split_2  = get_split()
#     connections.append((split_b.output(1), split_2.input(0)))

#     xor_2 = get_xor()
#     connections.append((split_1.output(1), xor_2.input(0)))
#     connections.append((split_2.output(0), xor_2.input(1)))

#     and_1 = get_and()
#     connections.append((xor_2.output(0), and_1.input(0)))
#     connections.append((split_2.output(1), and_1.input(1)))

#     output_a = get_input_output()
#     connections.append((split_1.output(0), output_a.input(0)))


#     sections = [input_a, split_b, xor_1, split_1, split_2, xor_2, and_1, output_a]
#     locations = [(1, 0),(3, 0), (0, 2), (1, 7), (5, 2), (2, 9), (4, 14), (1, 16)]

#     board = rasterize(sections, connections, locations)

#     return Section_Data([1, 4], [1, 5], board)

# def get_full_adder():
#     connections = []

#     input_a = get_input_output()
#     input_b = get_input_output()

#     half_adder_1 = get_half_adder()
#     connections.append((input_b.output(0), half_adder_1.input(1)))
#     connections.append((input_a.output(0), half_adder_1.input(0)))
    
#     carry_in = get_input_output()
#     half_adder_2 = get_half_adder()
#     connections.append((carry_in.output(0), half_adder_2.input(0)))
#     connections.append((half_adder_1.output(0), half_adder_2.input(1)))
    
#     or_1 = get_or()
#     connections.append(((half_adder_2.output(1), or_1.input(0))))
#     connections.append(((half_adder_1.output(1), or_1.input(1))))
    
#     output_a = get_input_output()
#     connections.append(((half_adder_2.output(0), output_a.input(0))))

#     sections = [input_a, input_b, half_adder_1, carry_in, half_adder_2, or_1, output_a]
#     locations = [(4,0), (7,0), (3, 3), (1, 0), (0, 20), (5, 37), (1, 39)]

#     board = rasterize(sections, connections, locations)

#     return Section_Data([1, 4, 7], [1, 6], board)


# def get_four_adder():
#     connections = []

#     adder_1 = get_full_adder()
    
#     adder_2 = get_full_adder()
#     connections.append((adder_1.output(1), adder_2.input(0)))

#     adder_3 = get_full_adder()
#     connections.append((adder_2.output(1), adder_3.input(0)))

#     adder_4 = get_full_adder()
#     connections.append((adder_3.output(1), adder_4.input(0)))


#     sections = [adder_1, adder_2, adder_3, adder_4]
#     locations = [(0,1), (12, 1), (24, 1), (36, 1)]

#     board = rasterize(sections, connections, locations)

#     return Blueprint([], [], board) #TODO finish finding input and outputs

# def get_rs_latch():
#     connections = []