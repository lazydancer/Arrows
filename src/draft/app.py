import draft.blueprints.defaults as defaults

import sys, os
sys.path.append(os.path.abspath('../../bridge_python_side'))
from bridge_python_side import adapter


def main():
    
    section = defaults.get_cross()
    board = section.rasterize()
    adapter.send_board(board)



    