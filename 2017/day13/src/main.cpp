#include "day13.h"
#include <fstream>
#include <iostream>
#include <print>
#include <span>

namespace {
void printError(const std::exception &e, std::string &str) {
    str += e.what();

    try {
        std::rethrow_if_nested(e);
    } catch (const std::exception &nested) {
        str += ": ";
        printError(nested, str);
    } catch (...) {
        str += ": <unknown exception>";
    }
}
} // namespace

int main(int argc, char *argv[]) {
    try {
        using namespace std::string_view_literals;
        std::span args{argv, static_cast<std::size_t>(argc)};

        if (args.size() < 2) {
            auto programName{[&]() {
                if (!args.empty()) {
                    return std::string_view{args[0]};
                }
                return "<unknown>"sv;
            }()};
            std::println(std::cerr, "Usage: {} <input file>", programName);
            throw std::runtime_error{"No input provided"};
        }

        std::ifstream inputFile{args[1]};
        if (!inputFile) {
            throw std::runtime_error{
                std::format("Failed to open `{}`", args[1])};
        }

        auto firewall{[&]() {
            try {
                return readInput(inputFile);
            } catch (...) {
                std::throw_with_nested(std::runtime_error{
                    std::format("Failed to read `{}`", args[1])});
            }
        }()};

        auto severity{firewall.getSeverity()};

        std::println("=== Part 1 ===\n"
                     "Severity: {}",
                     severity);

        auto delay{firewall.getDelay()};
        if (!delay) {
            throw std::runtime_error{"Failed to find delay"};
        }

        std::println("\n"
                     "=== Part 2 ===\n"
                     "Delay: {}",
                     *delay);
    } catch (std::exception &e) {
        std::string message{"Error: "};
        printError(e, message);
        std::println(std::cerr, "{}", message);
        return EXIT_FAILURE;
    }

    return 0;
}
