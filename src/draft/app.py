import draft.blueprints.defaults as defaults

import sys, os
sys.path.append(os.path.abspath('../../bridge_python_side'))
from bridge_python_side import adapter

from draft.blueprints.adj_level_raster import test_adj_levels_raster


def main():
    
    section = test_adj_levels_raster()
    adapter.send_board(section)



    