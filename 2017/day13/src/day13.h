#pragma once

#include <istream>
#include <optional>
#include <vector>

class Firewall {
  public:
    Firewall(std::vector<std::optional<std::size_t>> &&ranges);

    [[nodiscard]] std::size_t getSeverity() const;

    [[nodiscard]] std::optional<std::size_t> getDelay() const;

  private:
    std::vector<std::optional<std::size_t>> mRanges;
};

Firewall readInput(std::istream &input);
