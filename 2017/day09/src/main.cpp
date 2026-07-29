// I don't know why I wrote this one in C++

#include "day9.h"
#include <fstream>
#include <iostream>
#include <print>
#include <span>
#include <sstream>

int main(int argc, const char *argv[]) {
  const std::span args{argv, static_cast<std::size_t>(argc)};

  const auto *programName = "<unknown>";
  if (!args.empty()) {
    programName = args[0];
  }

  if (args.size() < 2) {
    std::println(std::cerr, "Usage: {} <input file>", programName);
    return 1;
  }

  std::ifstream programFile{args[1]};
  if (!programFile) {
    std::println(std::cerr, "Failed to open `{}` for reading", args[1]);
    return 1;
  }

  std::stringstream programStream;
  programStream << programFile.rdbuf();
  const auto programBuffer{programStream.str()};

  std::string_view trimmed_view{programBuffer};
  while (!trimmed_view.empty() &&
         std::isspace(static_cast<unsigned char>(trimmed_view[0])) != 0) {
    trimmed_view.remove_prefix(1);
  }
  while (!trimmed_view.empty() &&
         std::isspace(static_cast<unsigned char>(
             trimmed_view[trimmed_view.size() - 1])) != 0) {
    trimmed_view.remove_suffix(1);
  }

  std::string program{trimmed_view};

  const auto stream{Stream::create(std::move(program))};
  if (!stream) {
    std::println(std::cerr, "Failed to read stream: {}",
                 toString(stream.error()));
    return 1;
  }

  const auto score{stream->scoreGroups()};
  const auto count{stream->countGarbage()};

  std::println("=== Part 1 ===\n"
               "Score: {}\n"
               "\n"
               "=== Part 2 ===\n"
               "Count: {}",
               score, count);

  return 0;
}
