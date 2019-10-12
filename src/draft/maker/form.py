'''
connection = 
[
    (section_a, output_a)
    (section_b, output_b)
]


'''
#from draft.maker.topological_sort import topological_sort
from topological_sort import topological_sort

def find_overlaps(lines):
    for line_a in lines:
        for line_b in lines:
            if line_a is line_b:
                continue

            if line_b[0][1] > line_a[0][1] and line_b[1][1] < line_a[1][1]:
                print('Cross of:', line_a, line_b)

            if line_b[0][1] < line_a[0][1] and line_b[1][1] > line_a[1][1]:
                print('Cross of:', line_a, line_b)



def draw_lines(levels, connections):
    '''
    returns connections
    '''
    level_left = levels[0]
    level_right = levels[1]

    left = ports_left(level_left, connections)
    right = ports_right(level_right, connections)

    result = []
    for conn in connections:
        a, b = conn
        left_idx = left.index(a)
        right_idx = right.index(b)
        result.append([(0, left_idx), (1, right_idx)])

    return result


def ports_left(level, connections):
    result = []
    for section in level:
        result += sorted(list(map(
            lambda x: x[0],
            filter(
                lambda x: x[0][0] == section,
                connections
            ))))

    return result


def ports_right(level, connections):
    result = []
    for section in level:
        result += sorted(list(map(
            lambda x: x[1],
            filter(
                lambda x: x[1][0] == section,
                connections
            ))))

    return result


def form(connections):
    '''
    Forms a valid dependency graph
    '''

    adjacent_sections = [[port[0] for port in conn] for conn in connections]

    return topological_sort(adjacent_sections)

    print(levels)



def test():
    connections = [
        [(0, 0), (1, 1)],
        [(0, 1), (1, 0)],
    ]

    levels = form(connections)
    print(levels)
    lines = draw_lines(levels, connections)
    print(lines)
    find_overlaps(lines)

test()
