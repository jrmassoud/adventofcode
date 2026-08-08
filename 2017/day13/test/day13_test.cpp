#include "day13.h"
#include <catch2/catch_test_macros.hpp>
#include <sstream>

TEST_CASE("Sample input") {
    std::istringstream input{"0: 3\n"
                             "1: 2\n"
                             "4: 4\n"
                             "6: 4"};

    auto firewall{readInput(input)};

    REQUIRE(firewall.getSeverity() == 24);
    REQUIRE(firewall.getDelay() == 10);
}
