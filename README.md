
![Arrows](https://raw.githubusercontent.com/lazydancer/Arrows/master/docs/arrows_logo.png)

## About

**Arrows** is a celluar automata

![Arrows Demo](https://github.com/lazydancer/Arrows/raw/master/docs/arrows_demo.gif)

## How to use

There are two sides of the program. The largest dependency is ggez, which provides window and graphics management. Python talks to Rust through a C interface, where ccfi is needed for this to occur.

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


Rust includes all of the logic and graphics. Where a "Board" object can be seny to rust and displayed and run. The python code contains is higher level drafter, where more complex boards can be designed. Right now in the drafter, there is a topological function to arrange sub-blocks and a basic router for wires using breadth first search. Improving this drafter is an interesting challenge as wires cannot cross, making interesting router challenges.


## Code Structure
```
src
├── bridge_python_side
│   ├── adapter.py
│   └── comm.py
├── bridge_rust_side
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── draft
│   ├── app.py
│   ├── defaults
│   │   └── defaults.py
│   ├── maker
│   │   ├── connector.py
│   │   ├── form.py
│   │   ├── raster.py
│   │   └── topological_sort.py
│   └── model
│       ├── block.py
│       └── section.py
├── logic
│   ├── Cargo.toml
│   └── src
│       ├── block.rs
│       ├── board.rs
│       ├── lib.rs
│       └── pos.rs
└── view
    ├── Cargo.toml
    ├── resources
    │   ├── arrow_down_active.png
    │   ├── arrow_down_inactive.png
    │   ├── arrow_left_active.png
    │   ├── arrow_left_inactive.png
    │   ├── arrow_right_active.png
    │   ├── arrow_right_inactive.png
    │   ├── arrow_up_active.png
    │   └── ...
    └── src
        ├── assets.rs
        └── lib.rs
```
