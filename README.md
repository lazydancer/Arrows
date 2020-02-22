
![Arrows](https://raw.githubusercontent.com/lazydancer/Arrows/master/docs/arrows_logo.png)

## About

**Arrows** is an logic automata

![Arrows Demo](https://raw.githubusercontent.com/lazydancer/Arrows/master/examples/arrows_demo.gif)

## How to use

There are two sides of the program. The largest dependecy is the need for ggez which provides window and graphics management. Python talks to Rust through a C interface, where ccfi is needed for this to occur.

**Python ">3.0"**
ccfi ">1.13"

**Rust "2018 Edition"**
ggez = "0.5.1"
rand = "0.7"
mint = "0.5"
cgmath = { version = "0.17", features = ["mint"]}


Build and run the program through make 

```
make run
```

## How it works


Rust includes all of the logic and graphics. Where a "Board" object can be send to rust, it will display and run this board. Python is higher level drafter, where more complex boards can be designed more quickly. Right now in the drafter, there is a topological funciton to arrange subblock and a basic router for wires using breadth first search. Improving this drafter is an interesting challenge as wires cannot cross, making interesting routering challenges.

I have written more about this on my [website](pucula.com/projects/Arrows)  
