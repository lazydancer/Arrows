
#from draft.maker.topological_sort import topological_sort
import unittest
from topological_sort import topological_sort


def find_overlaps(lines):
    overlaps = []
    while len(lines) >= 2:
        line_a = lines.pop()
        for line_b in lines:
            lines_cross_check = line_b[0][1] > line_a[0][1] and line_b[1][1] < line_a[1][1]
            lines_cross_reverse = line_b[0][1] < line_a[0][1] and line_b[1][1] > line_a[1][1]

            if lines_cross_check or lines_cross_reverse:
                overlaps.append([line_a, line_b])
                print('Cross of:', line_a, line_b)

    return overlaps


def draw_lines(levels, connections):
    level_left = levels[0]
    level_right = levels[1]

    left = ports_left(level_left, connections)
    right = ports_right(level_right, connections)

    print(left)

    lines = []
    for conn in connections:
        a, b = conn
        left_idx = left.index(a)
        right_idx = right.index(b)
        lines.append([(0, left_idx), (1, right_idx)])

    overlaps = find_overlaps(lines)

    return overlaps


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


def determine_levels(connections):
    adjacent_sections = [[port[0] for port in conn] for conn in connections]

    return topological_sort(adjacent_sections)

    print(levels)


def test():
    connections = [
        [(0, 0), (1, 1)],
        [(0, 1), (1, 0)],
    ]

    levels = determine_levels(connections)
    print(levels)
    lines = draw_lines(levels, connections)
    print(lines)


test()


class Test(unittest.TestCase):

    def test_simple_overlap(self):
        self.assertEqual([['f', 'a'], ['b', 'd'], ['e']],
                         topological_sort('fb bc ab ad de be'.split()))
