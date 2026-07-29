#include "day9.h"
#include <format>

namespace {
template <class... Ts> struct overloaded : Ts... {
  using Ts::operator()...;
};
} // namespace

std::expected<Stream, StreamCreateError> Stream::create(std::string &&data) {
  if (data.size() < 2 || data[0] != '{' || data[data.size() - 1] != '}') {
    return std::unexpected{stream_create_error::NotInGroup{}};
  }

  auto it{data.begin()};
  std::size_t groupDepth{0};
  auto inGarbage{false};
  while (it != data.end()) {
    if (!inGarbage) {
      if (*it == '{') {
        groupDepth += 1;
      }

      if (*it == '}') {
        if (groupDepth == 0) {
          return std::unexpected{stream_create_error::ExtraCloseBracket{
              .idx = static_cast<std::size_t>(it - data.begin())}};
        }
        groupDepth -= 1;
      }

      if (*it == '<') {
        inGarbage = true;
      }
    } else {
      if (*it == '!') {
        it++;
        if (it == data.end()) {
          break;
        }
        it++;
        continue;
      }

      if (*it == '>') {
        inGarbage = false;
      }
    }
    it++;
  }

  if (groupDepth != 0) {
    return std::unexpected{
        stream_create_error::UnclosedGroup{.count = groupDepth}};
  }

  return Stream{std::move(data)};
}

[[nodiscard]] std::size_t Stream::scoreGroups() const {
  auto it{mData.begin()};
  std::size_t groupDepth{0};
  auto inGarbage{false};
  std::size_t score{0};

  while (it != mData.end()) {
    if (!inGarbage) {
      if (*it == '{') {
        groupDepth += 1;
        score += groupDepth;
      }

      if (*it == '}') {
        groupDepth -= 1;
      }

      if (*it == '<') {
        inGarbage = true;
      }
    } else {
      if (*it == '!') {
        it++;
        if (it == mData.end()) {
          break;
        }
        it++;
        continue;
      }

      if (*it == '>') {
        inGarbage = false;
      }
    }
    it++;
  }

  return score;
}

[[nodiscard]] std::size_t Stream::countGarbage() const {
  std::size_t count{0};
  auto inGarbage{false};
  auto it{mData.begin()};

  while (it != mData.end()) {
    if (inGarbage) {
      if (*it == '!') {
        it++;
        if (it == mData.end()) {
          break;
        }
        it++;
        continue;
      }

      if (*it == '>') {
        inGarbage = false;
      } else {
        count++;
      }
    } else {
      if (*it == '<') {
        inGarbage = true;
      }
    }
    it++;
  }

  return count;
}

Stream::Stream(std::string &&data) : mData{std::move(data)} {};
std::string toString(const StreamCreateError &e) {
  return std::visit(
      overloaded{[](stream_create_error::NotInGroup /*e*/) {
                   return std::string("Data is not enclosed in a group.");
                 },
                 [](stream_create_error::UnclosedGroup e) {
                   return std::format("Unclosed group at depth {}", e.count);
                 },
                 [](stream_create_error::ExtraCloseBracket e) {
                   return std::format("Extra close bracket at index {}", e.idx);
                 }},
      e);
}
