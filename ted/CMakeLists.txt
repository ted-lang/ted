cmake_minimum_required(VERSION 3.0.0)
project(tedc)

add_executable(ted "src/main.cpp")
if (USE_THREADS)
    target_link_libraries(ted PRIVATE Threads::Threads)
endif()
