cmake_minimum_required(VERSION 3.16)

add_library(utils
file_utils.c
json_utils.c
)

target_include_directories(utils PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../headers)
