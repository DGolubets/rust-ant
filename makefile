DIR_BIN = bin/x86_64
DIR_OUT = target/release
DIR_DIST = target/dist

default: build dist

build:
	cargo rustc --release -- -C link_args="-Wl,--subsystem,windows"

dist:
	rm $(DIR_DIST) -rf
	mkdir $(DIR_DIST)
	cp $(DIR_BIN)/*.dll $(DIR_DIST)
	cp $(DIR_OUT)/*.exe $(DIR_DIST)
	cp resources $(DIR_DIST) -r
