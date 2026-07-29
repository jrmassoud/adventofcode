add_library(aoc2017d09_lib STATIC)

target_sources(aoc2017d09_lib
  PRIVATE
    day9.cpp
  PUBLIC
    include/day9.h
)

target_compile_features(aoc2017d09_lib PUBLIC cxx_std_23)

target_include_directories(aoc2017d09_lib PUBLIC include)

enable_warnings(aoc2017d09_lib)

add_executable(aoc2017d09)

target_sources(aoc2017d09 PRIVATE main.cpp)

target_link_libraries(aoc2017d09 PRIVATE aoc2017d09_lib)

target_compile_features(aoc2017d09 PRIVATE cxx_std_23)

enable_warnings(aoc2017d09)
