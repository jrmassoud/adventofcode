add_executable(aoc2017d09_test)

target_sources(aoc2017d09_test PRIVATE
  day9_test.cpp
)

target_link_libraries(aoc2017d09_test PRIVATE
  aoc2017d09_lib
  Catch2::Catch2WithMain
)

target_compile_features(aoc2017d09_test PRIVATE cxx_std_23)

enable_warnings(aoc2017d09_test)

catch_discover_tests(aoc2017d09_test)
