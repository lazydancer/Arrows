from simplecircuit.engine.model import Model
from simplecircuit.engine.controller import Controller

from simplecircuit.blueprints.blueprint import Blueprint
import simplecircuit.blueprints.defaults as defaults
from simplecircuit.blueprints.connector import connect
from simplecircuit.blueprints.raster import rasterize

def run():
    model = Model(50)
    controller = Controller(model)

    half_adder = defaults.get_four_adder()

    board = rasterize([half_adder], [], [(0, 0)])

    model.apply(board, 0, 0)

    model.run()

