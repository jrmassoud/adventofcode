add_library(aoc2017d13_lib OBJECT)

target_sources(aoc2017d13_lib
  PRIVATE
    day13.cpp
  PUBLIC
    day13.h
)

target_compile_features(aoc2017d13_lib PUBLIC cxx_std_23)

target_include_directories(aoc2017d13_lib PUBLIC .)

enable_warnings(aoc2017d13_lib)

add_executable(aoc2017d13)

target_sources(aoc2017d13 PRIVATE main.cpp)

target_link_libraries(aoc2017d13 PRIVATE aoc2017d13_lib)

target_compile_features(aoc2017d13 PRIVATE cxx_std_23)

enable_warnings(aoc2017d13)
