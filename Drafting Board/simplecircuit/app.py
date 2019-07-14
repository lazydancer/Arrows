from simplecircuit.engine.model import Model
from simplecircuit.engine.controller import Controller

'''
def run_test():


    model = Model(50)
    controller = Controller(model)

    a = HalfAdder()
    model.apply(a.get(),0,0)

    model.run()
'''
from simplecircuit.blueprints.blueprint_logic import run

def run_test():
    run()




    