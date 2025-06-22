#include "cpp-lib.hpp"

#include <iostream>
#include <stdexcept>

int cpp_lib_add(int a, int b) {
  std::cout << "cpp_lib_add " << a << " + " << b << '\n';
  return a + b;
}

int cpp_lib_add_throws(int a, int b) {
  std::cout << "cpp_lib_add_throws " << a << " + " << b << '\n';
  if (a) {
    throw std::runtime_error{"im throwing"};
  }
  return a + b;
}
