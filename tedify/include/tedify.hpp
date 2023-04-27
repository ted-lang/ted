/**
 * @file tedify.hpp
 * @author Wuqiong Zhao (me@wqzhao.org)
 * @brief Tedify Implementation
 * @version 0.1.0
 * @date 2023-04-27
 *
 * @copyright Copyright (c) 2023 Wuqiong Zhao (Teddy van Jerry)
 *
 */

#ifndef _TEDIFY_HPP_
#define _TEDIFY_HPP_

#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

/**
 * @brief Tedify the file.
 *
 * @details Change all instances of `@teddy` to `🧸` not within comments.
 * @param input Input file path.
 * @param output Output file path.
 * @retval true Tedification succeeds.
 * @retval false Tedification fails.
 */
static bool tedify(const std::string& input, const std::string& output) {
    std::ifstream in(input);
    if (!in.is_open()) {
        std::cerr << "Could not open input file " << input << std::endl;
        return false;
    }
    std::vector<std::string> lines;
    std::string buf;
    while (std::getline(in, buf)) lines.push_back(buf);
    in.close();

    std::ofstream out(output);
    if (!out.is_open()) {
        std::cerr << "Could not open output file " << output << std::endl;
        return false;
    }
    // Look behind is not supported by `std::regex`, so `/(?<!#.*)@teddy/` is not valid.
    // Otherwise, we could have used `std::regex_replace` to deal with the whole document.
    for (auto& line : lines) {
        // Find the start of the comment.
        // This is not strictly correct, but applies to most of scenarios though.
        auto i        = line.find_last_of('#');
        auto end_iter = i == std::string::npos ? line.end() : line.begin() + i;
        std::regex_replace(std::ostreambuf_iterator<char>(out), line.begin(), end_iter, std::regex("@teddy\\b"), "🧸");
        if (i != std::string::npos) out << line.substr(i) << '\n'; // the comment part
    }
    out.close();
    return true;
}

#endif
