from simplecircuit.engine.view import TextView
from simplecircuit.common.block import Block, block_groups

class Controller:

    def __init__(self, model):
        self.model = model
        self.view = TextView(self, model)

    def dispose(self):
        self.view.dispose()

    def click(self, x, y):
        block = self.model.get_block(x-1, y-1)
        if block in block_groups['wires']:
            new_block = Block.negate_right
        if block in block_groups['negates']:
            new_block = Block.split_right
        if block in block_groups['splits']:
            new_block = Block.space
        if block in block_groups['space']:
            new_block = Block.wire_right
        
        self.model.set_block(new_block, x-1, y-1)

    def rotate_block(self, x, y):
        block = self.model.get_block(x-1, y-1)
       
        new_block = block + 1

        if block == Block.wire_down:
            new_block = Block.wire_left
        if block == Block.negate_down:
            new_block = Block.negate_left
        if block == Block.split_down:
            new_block = Block.split_left
        if block == 13:
            new_block = Block.space
 
        self.model.set_block(new_block, x-1, y-1)        

    def quit(self):
        self.view.dispose()
        self.model.quit()