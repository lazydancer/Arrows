from cffi import FFI

ffi = FFI()
ffi.cdef("""
    typedef void* board;

    board board_new();
    void board_free(board);

    void board_add_block(board, int x, int y, int blocktype);
    void board_start(board);
""")

C = ffi.dlopen('../rust_side/target/debug/librust_side.so')

print("From Python:", C.board_new())


class Board:
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

with Board() as board:
    board.add_block(4, 3, 2)
    board.start()