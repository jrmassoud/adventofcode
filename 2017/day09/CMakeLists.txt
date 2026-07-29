cmake_minimum_required(VERSION 4.4)
project(aoc2017d09)

find_package(Catch2 REQUIRED VERSION 3)
include(CTest)
include(Catch)

function(enable_warnings target)
  target_compile_options(${target} PRIVATE
    $<$<CXX_COMPILER_ID:MSVC>:/W4>
    $<$<CXX_COMPILER_ID:GNU,Clang,AppleClang>:-Wall -Wextra -pedantic>
  )
endfunction()

add_subdirectory(src)
add_subdirectory(test)
