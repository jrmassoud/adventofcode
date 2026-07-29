#pragma once

#include <cstddef>
#include <expected>
#include <string>
#include <variant>

namespace stream_create_error {
struct NotInGroup {};

struct UnclosedGroup {
  std::size_t count;
};

struct ExtraCloseBracket {
  std::size_t idx;
};
} // namespace stream_create_error

using StreamCreateError = std::variant<stream_create_error::NotInGroup,
                                       stream_create_error::UnclosedGroup,
                                       stream_create_error::ExtraCloseBracket>;

std::string toString(const StreamCreateError &e);

class Stream {
public:
  static std::expected<Stream, StreamCreateError> create(std::string &&data);

  [[nodiscard]] std::size_t scoreGroups() const;

  [[nodiscard]] std::size_t countGarbage() const;

private:
  Stream(std::string &&data);

  std::string mData;
};
