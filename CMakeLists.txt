cmake_minimum_required(VERSION 3.22 FATAL_ERROR)

project(rust-ffi)

add_compile_options(-Wall -Werror -Wpedantic)

add_subdirectory(libs/c-lib)
add_subdirectory(libs/cpp-lib)
