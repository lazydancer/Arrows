from bridge_python_side import adapter
import draft.defaults.defaults as defaults

import sys
import os
sys.path.append(os.path.abspath('../../bridge_python_side'))


def main():
    section = defaults.get_and()
    arrows = section.rasterize()
    adapter.send_sparse(arrows)
