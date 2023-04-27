cmake_minimum_required(VERSION 3.0.0)
project(tedify)

include_directories("include" "../include")
add_executable(tedify "src/main.cpp")
if (USE_THREADS)
    target_link_libraries(tedify PRIVATE Threads::Threads)
endif()

if (TED_TESTS)
    add_test(NAME tedify_v       COMMAND tedify -v)
    add_test(NAME tedify_h       COMMAND tedify -h)
    add_test(NAME tedify_replace COMMAND tedify ${CMAKE_SOURCE_DIR}/tests/hello_world_untedified.ted ${CMAKE_SOURCE_DIR}/tests/hello_world_tedified.ted.test)
endif()
