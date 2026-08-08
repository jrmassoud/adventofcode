#include "day13.h"
#include <format>
#include <ranges>

Firewall::Firewall(std::vector<std::optional<std::size_t>> &&ranges)
    : mRanges{std::move(ranges)} {}

[[nodiscard]] std::size_t Firewall::getSeverity() const {
    std::size_t severity{0};
    for (auto [i, v] : std::views::enumerate(mRanges)) {
        if (v) {
            if (*v == 1 || i % (*v * 2 - 2) == 0) {
                severity += i * *v;
            }
        }
    }
    return severity;
}

[[nodiscard]] std::optional<std::size_t> Firewall::getDelay() const {
    std::size_t delay{0};
    while (true) {
        bool caught{false};
        for (auto [i, v] : std::views::enumerate(mRanges)) {
            if (v) {
                if (*v == 1) {
                    return std::nullopt;
                }
                if ((i + delay) % (*v * 2 - 2) == 0) {
                    caught = true;
                    break;
                }
            }
        }
        if (!caught) {
            return delay;
        }
        delay++;
    }
}

Firewall readInput(std::istream &input) {
    std::string line;
    std::vector<std::optional<std::size_t>> output;

    std::optional<std::size_t> previous{std::nullopt};
    std::size_t no{1};

    while (std::getline(input, line)) {
        std::string_view lineView{line};
        while (!lineView.empty() &&
               std::isspace(static_cast<unsigned char>(lineView[0])) != 0) {
            lineView.remove_prefix(1);
        }
        while (!lineView.empty() && std::isspace(static_cast<unsigned char>(
                                        lineView[lineView.size() - 1])) != 0) {
            lineView.remove_suffix(1);
        }

        std::size_t depth{0};
        auto [depthEnd, depthEc] =
            std::from_chars(lineView.begin(), lineView.end(), depth);
        if (depthEc != std::errc{}) {
            try {
                throw std::system_error{std::make_error_code(depthEc)};
            } catch (...) {
                std::throw_with_nested(std::runtime_error{
                    std::format("Failed to parse depth on line {}", no)});
            }
        }

        auto colonIdx{static_cast<std::size_t>(
            std::distance(lineView.begin(), depthEnd))};

        if (colonIdx + 2 >= lineView.size() || lineView[colonIdx] != ':' ||
            lineView[colonIdx + 1] != ' ') {
            throw std::runtime_error(
                std::format("Invalid format on line {}", no));
        }

        std::size_t range{0};
        auto rangeStart = colonIdx + 2;
        auto [rangeEnd, rangeEc] =
            std::from_chars(&lineView[rangeStart], lineView.end(), range);
        if (rangeEc != std::errc{}) {
            try {
                throw std::system_error{std::make_error_code(rangeEc)};
            } catch (...) {
                std::throw_with_nested(std::runtime_error{
                    std::format("Failed to parse range on line {}", no)});
            }
        }
        if (rangeEnd != lineView.end()) {
            throw std::runtime_error{
                std::format("Expected EOF on line {}", no)};
        }

        if (previous) {
            if (depth <= *previous) {
                throw std::runtime_error{
                    std::format("Depth is not increasing on line {}", no)};
            }
            for (std::size_t i{*previous + 1}; i < depth; ++i) {
                output.emplace_back(std::nullopt);
            }
        } else {
            for (std::size_t i{0}; i < depth; ++i) {
                output.emplace_back(std::nullopt);
            }
        }
        output.emplace_back(range);
        previous = depth;

        no += 1;
    }

    if (!input.good() && !input.eof()) {
        throw std::runtime_error{std::format("Failed to read line {}", no)};
    }

    return Firewall{std::move(output)};
}
