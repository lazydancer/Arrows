from cffi import FFI
from os import path

ffi = FFI()
ffi.cdef("""
    typedef void* board;

    board board_new();
    void board_free(board);

    void board_add_block(board, int x, int y, int blocktype);
    void board_start(board);
""")

dl_path = path.join(path.dirname(__file__), '../bridge_rust_side/target/release/librust_side.so')
C = ffi.dlopen(dl_path)

class Engine_Board:
    def __init__(self):
        self.__obj = C.board_new()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        C.board_free(self.__obj)
        self.__obj = None

    def add_block(self, x, y, block):
        C.board_add_block(self.__obj, x, y, block)

    def start(self):
        C.board_start(self.__obj)

# Reference on how to use
# with Engine_Board() as board:
#     board.add_block(4, 3, 2)
#     board.start()