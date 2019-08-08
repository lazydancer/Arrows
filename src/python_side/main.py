from cffi import FFI

ffi = FFI()
ffi.cdef("""
    typedef struct {
        double x, y;
    } pos_t;

    int length(const pos_t *pos);

    int doubl(int);
    void start_sim();
""")

C = ffi.dlopen('../rust_side/target/debug/librust_side.so')

pos = ffi.new("pos_t *")
pos.x = 7
pos.y = 2

print(C.length(pos))
C.start_sim()