from draft.engine.model import Model
from draft.engine.controller import Controller

from draft.blueprints.blueprint import Blueprint
import draft.blueprints.defaults as defaults
from draft.blueprints.connector import connect
from draft.blueprints.raster import rasterize

def run():
    model = Model(50)
    controller = Controller(model)

    half_adder = defaults.get_four_adder()

    board = rasterize([half_adder], [], [(0, 0)])

    model.apply(board, 0, 0)

    model.run()

