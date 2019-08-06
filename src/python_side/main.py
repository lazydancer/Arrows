from cffi import FFI

ffi = FFI()
ffi.cdef("int doubl(int);")
C = ffi.dlopen('../rust_side/target/release/librust_side.so')

print(C.doubl(9))