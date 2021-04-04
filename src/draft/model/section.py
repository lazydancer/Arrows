import abc
from draft.maker.raster import raster

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

    def width(self): 
        return max([x for x, y in self.data]) + 1

    def height(self):
        return max([y for x, y in self.data]) + 1

    def rasterize(self):
        return self.data

class Section_Section(Section):
    def __init__(self, data):
        self.data = data

    def rasterize(self):
        breakpoint()

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
        return raster(self._connections)
