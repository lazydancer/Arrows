from cffi import FFI

ffi = FFI()
ffi.cdef("""
    int doubl(int);
    void start_sim();
""")
C = ffi.dlopen('../rust_side/target/debug/librust_side.so')

print(C.start_sim())