class Section:
    def __init__(self, inputs, outputs, data):
        self.inputs = inputs
        self.outputs = outputs
        self.data = data
        self.sections = []
        self.connections = []

    def input(self, input_num):
        return (self, input_num)
    
    def output(self, output_num):
        return (self, output_num)