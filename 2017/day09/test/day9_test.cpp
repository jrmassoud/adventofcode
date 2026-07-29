#include "day9.h"
#include <catch2/catch_test_macros.hpp>

TEST_CASE( "scoreGroups works" ) {
  auto empty{Stream::create("{}")};
  REQUIRE(empty);
  REQUIRE(empty->scoreGroups() == 1);

  auto nested{Stream::create("{{{}}}")};
  REQUIRE(nested);
  REQUIRE(nested->scoreGroups() == 6);

  auto doubleNested{Stream::create("{{},{}}")};
  REQUIRE(doubleNested);
  REQUIRE(doubleNested->scoreGroups() == 5);

  auto majorNested{Stream::create("{{{},{},{{}}}}")};
  REQUIRE(majorNested);
  REQUIRE(majorNested->scoreGroups() == 16);

  auto garbage{Stream::create("{<a>,<a>,<a>,<a>}")};
  REQUIRE(garbage);
  REQUIRE(garbage->scoreGroups() == 1);

  auto nestedGarbage{Stream::create("{{<ab>},{<ab>},{<ab>},{<ab>}}")};
  REQUIRE(nestedGarbage);
  REQUIRE(nestedGarbage->scoreGroups() == 9);

  auto nonCancelled{Stream::create("{{<!!>},{<!!>},{<!!>},{<!!>}}")};
  REQUIRE(nonCancelled);
  REQUIRE(nonCancelled->scoreGroups() == 9);

  auto cancelled{Stream::create("{{<a!>},{<a!>},{<a!>},{<ab>}}")};
  REQUIRE(cancelled);
  REQUIRE(cancelled->scoreGroups() == 3);
}

TEST_CASE("countGarbage works") {
  auto empty{Stream::create("{<>}")};
  REQUIRE(empty);
  REQUIRE(empty->countGarbage() == 0);

  auto random{Stream::create("{<random characters>}")};
  REQUIRE(random);
  REQUIRE(random->countGarbage() == 17);

  auto opening{Stream::create("{<<<<>}")};
  REQUIRE(opening);
  REQUIRE(opening->countGarbage() == 3);

  auto ignore{Stream::create("{<{!>}>}")};
  REQUIRE(ignore);
  REQUIRE(ignore->countGarbage() == 2);

  auto ignoreToZero{Stream::create("{<!!>}")};
  REQUIRE(ignoreToZero);
  REQUIRE(ignoreToZero->countGarbage() == 0);

  auto extremeIgnore{Stream::create("{<!!!>>}")};
  REQUIRE(extremeIgnore);
  REQUIRE(extremeIgnore->countGarbage() == 0);

  auto randomChars{Stream::create("{<{o\"i!a,<{i<a>}")};
  REQUIRE(randomChars);
  REQUIRE(randomChars->countGarbage() == 10);
}
