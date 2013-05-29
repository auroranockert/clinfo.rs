all: clinfo

clinfo: clinfo.rs cl.rs/libcl-766bd36a22c24c8-0.1.dylib
	rust build clinfo.rs -L cl.rs -L opencl.rs -L string.rs -L vector.rs

cl.rs/cl.rs:
	git clone https://github.com/JensNockert/cl.rs.git

cl.rs/libcl-766bd36a22c24c8-0.1.dylib: cl.rs/cl.rs opencl.rs/libopencl-6efee7f734c8ee7-0.1.dylib string.rs/libstring-be6f5ba71facfd3-0.1.dylib vector.rs/libvector-bfbddd23f529632-0.1.dylib
	rust build cl.rs/cl.rs -L opencl.rs -L string.rs -L vector.rs

opencl.rs/opencl.rs:
	git clone https://github.com/JensNockert/opencl.rs.git

opencl.rs/libopencl-6efee7f734c8ee7-0.1.dylib: opencl.rs/opencl.rs
	rust build opencl.rs/opencl.rs

string.rs/string.rs:
	git clone https://github.com/JensNockert/string.rs.git

string.rs/libstring-be6f5ba71facfd3-0.1.dylib: string.rs/string.rs
	rust build string.rs/string.rs

vector.rs/vector.rs:
	git clone https://github.com/JensNockert/vector.rs.git

vector.rs/libvector-bfbddd23f529632-0.1.dylib: vector.rs/vector.rs
	rust build vector.rs/vector.rs

clean:
	rm -rf cl.rs opencl.rs string.rs vector.rs clinfo clinfo.dSYM