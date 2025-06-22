BUILD_DIR=$(CURDIR)/build
CORES=$(shell nproc)

.PHONY: all cmake-cfg c-lib cpp-lib

all: c-lib cpp-lib

cmake-cfg:
	cmake -S . -B $(BUILD_DIR) \
		-D CMAKE_BUILD_TYPE=Debug

c-lib: cmake-cfg
	cmake --build $(BUILD_DIR) -j$(CORES) -t c-lib

cpp-lib: cmake-cfg
	cmake --build $(BUILD_DIR) -j$(CORES) -t cpp-lib
