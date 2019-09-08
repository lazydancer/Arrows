from draft.engine.model import Model
from draft.engine.controller import Controller

from draft.blueprints.blueprint import Blueprint
import draft.blueprints.defaults as defaults
from draft.blueprints.connector import connect
from draft.blueprints.raster import rasterize

import sys, os
sys.path.append(os.path.abspath('../../python_side'))
from python_side import adapter


from draft.blueprints.adj_level_raster import test_adj_levels_raster

def run():
    model = Model(50)
    controller = Controller(model)

    half_adder = defaults.get_four_adder()


    #board = rasterize([half_adder], [], [(0, 0)])

    board = test_adj_levels_raster()

    model.apply(board, 0, 0)

    adapter.send_board(model.get_board())

    #model.run()

