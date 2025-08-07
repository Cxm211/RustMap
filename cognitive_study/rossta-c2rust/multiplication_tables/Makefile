main: ./main.o
	gcc -o main ./main.o

.c.o: 
	gcc -c $<

.cpp.o: 
	g++ -c $<

clean::
		rm -rf Makefile main c2rust crusts compile_commands.json Cargo.lock target src build.rs Cargo.lock Cargo.toml lib.rs main.o rust-toolchain.toml ../metrics