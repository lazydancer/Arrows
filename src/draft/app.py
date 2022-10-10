from bridge_python_side import adapter
from draft.block import Block
import draft.section as section

import sys
import os
sys.path.append(os.path.abspath('../../bridge_python_side'))


def main():
    arrows = section.flatten(section.s['cross'])
    print(arrows)
    adapter.send_sparse(arrows)
