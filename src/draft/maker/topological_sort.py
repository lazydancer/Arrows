from collections import defaultdict

def topological_sort(dependency_pairs):
    # Sort values subject to dependency constraints'

    '''Part 1: Set up the graph with each node having:
            num_heads: an int for arrows point to
            tails: a list of arrows going out
            heads a unique list of heads
    '''
    num_heads = defaultdict(int)   
    tails = defaultdict(list)       
    heads = []

    for h, t in dependency_pairs:
        num_heads[t] += 1
        if h in tails:
            tails[h].append(t)
        else:
            tails[h] = [t]
            heads.append(h)

    layer = [h for h in heads if h not in num_heads] #Calculates the first 'layer'


    '''Part 2: Go through each layer of the graph removing tails
        from the previous layer to show the next layer
    '''
    result = []    
    while layer != []:
        result.append(layer)

        next_layer = []
        for h in layer:
            # for each tail, subtract one from the head it is going to
            for t in tails[h]:
                num_heads[t] -= 1
                if not num_heads[t]:
                    next_layer.append(t)

        layer = next_layer        

    return result


import unittest
class Test(unittest.TestCase):
    
    def test_topological_sort(self):
        self.assertEqual([['f', 'a'], ['b', 'd'], ['c', 'e']], topological_sort('fb bc ab ad de be'.split()))
    

    def test_topological_sort_half_adder(self):
        self.assertEqual([['a'],['b', 'd'],['c'],['e'],['f']], topological_sort('ab bc ad ce de ef df'.split()))
