cmake_minimum_required(VERSION 4.3)
project(aoc2017d13)

find_package(Catch2 3 REQUIRED)
include(CTest)
include(Catch)

function(enable_warnings target)
  target_compile_options(${target} PRIVATE
    $<$<CXX_COMPILER_ID:MSVC>:/W4>
    $<$<CXX_COMPILER_ID:GNU,Clang,AppleClang>:-Wall -Wextra -pedantic -Wno-c2y-extensions>
  )
endfunction()

add_subdirectory(src)
add_subdirectory(test)
