import time

from draft.common.block import Block

import draft.engine.solver as solver

class Model:
    '''
    _board is an 2D array containing a number with a number representing a block
    _state is a 2D array of 4-item tuples, the tuple contains the output of the block  
    '''

    def __init__(self, size):
        self._board = [[Block.space for _ in range(size+2)] for _ in range(size+2)]
        self._state = [[(False, False, False, False) for x in range(size+2)] for x in range(size+2)]
        self.observers = []
    

    # Observer Code
    def registerObserver(self, o):
        self.observers.append(o)

    def removeObserver(self, o):
        if o in self.observers:
            self.observers.remove(o)

    def _event(self):
        for o in self.observers:
            o.notified(self._board, self._state)


    # Block Management
    def set_block(self, block, x, y):
        self._board[y+1][x+1] = block

    def apply(self, block, at_x, at_y):
        for j, y in enumerate(range(at_y, at_y + len(block))):
            for i, x in enumerate(range(at_x, at_x + len(block[0]))):
                if block[j][i] == None:
                    continue
                self._board[y+1][x+1] = block[j][i]

    def get_block(self, x, y):
        return self._board[y+1][x+1]
    
    def get_board(self):
        new_board = self._board[1:-1]

        for i in range(len(new_board)):
            new_board[i] = new_board[i][1:-1]

        return new_board

    def add_input(self, y, state):
        self._board[y+1][0] = Block.negate_right if state else Block.space  
        
    def get_output(self, y):
        return self._state[y+1][len(self._board)-2][2]


    # Simulation
    def run(self):
        self.isRun = True
 
        while(self.isRun):
            self._step()
            time.sleep(0.1)
            
    def _step(self):  
        self._state = solver.solve(self._board, self._state)
        self._event()

    def quit(self):
        if self.isRun is not None:
            self.isRun = False