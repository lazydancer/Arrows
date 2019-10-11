import draft.blueprints.defaults as defaults

import sys, os
sys.path.append(os.path.abspath('../../bridge_python_side'))
from bridge_python_side import adapter


def main():
    
    section = defaults.four_input_and()
    arrows = section.rasterize()
    adapter.send_sparse(arrows)

    #adapter.send_sparse(1)


    