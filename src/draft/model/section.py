import abc
from draft.blueprints.adj_level_raster import raster_topo

class Section(abc.ABC):
    '''
    Main 'Box' used in Arrows. 

    Section is split into two types

    Section_Data which contains actual rastized arrows

    Section_Flex contains other sections and there connections
    '''
    @abc.abstractmethod
    def rasterize(self):
        pass

class Section_Data(Section):
    def __init__(self, inputs, outputs, data):
        self.inputs = inputs # block where the input is at x=0
        self.outputs = outputs  # block where the output is at x= widht(data)
        self.data = data # 2D list with arrow data (rasterized)

    def input(self, input_num):
        return (self, input_num)
    
    def output(self, output_num):
        return (self, output_num)

    def rasterize(self):
        return self.data


class Section_Flex(Section):
    def __init__(self, flex_input, flex_outputs, connections):
        self.flex_input = flex_input # inputs is a list of (block, input num)
        self.flex_outputs = flex_outputs # outputs is a list of (block, ouput num)
        self._connections = connections # 2D list with arrow data (rasterized)

    def input(self, input_num):
        return (self, input_num)
    
    def output(self, output_num):
        return (self, output_num)

    def rasterize(self):
        return raster_topo(self._connections)
