import draft.blueprints.defaults as defaults

import sys, os
sys.path.append(os.path.abspath('../../bridge_python_side'))
from bridge_python_side import adapter

from draft.blueprints.adj_level_raster import topo_test 


def main():
    
    section = topo_test()
    adapter.send_board(section)



    