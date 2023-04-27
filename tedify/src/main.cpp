/**
 * @file main.cpp
 * @author Wuqiong Zhao (me@wqzhao.org)
 * @brief Tedify Main File
 * @version 0.1.0
 * @date 2023-04-27
 *
 * @copyright Copyright (c) 2023 Wuqiong Zhao (Teddy van Jerry)
 *
 */

#include "meta.hpp"
#include "tedify.hpp"

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cerr << "You need to provide a Ted file name." << std::endl;
        return 1;
    } else if (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0) {
        std::cout << "Tedify - Make Teddy Unicode From @teddy\n"
                     "Usage:\n   tedify <input file> <output file>\n"
                     "   Leave output file name empty to change the original file.\n";
        return 0;
    } else if (strcmp(argv[1], "-v") == 0 || strcmp(argv[1], "--version") == 0) {
        std::cout << "Tedify " << TED_VERSION_STRING << std::endl;
        return 0;
    } else if (argc != 2) { // 3 or more arguments
        if (!tedify(argv[1], argv[2])) return 3;
    } else { // 2 arguments, no output file
        if (!tedify(argv[1], argv[1])) return 2;
    }
    return 0;
}
