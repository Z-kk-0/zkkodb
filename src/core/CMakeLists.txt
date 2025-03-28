cmake_minimum_required(VERSION 3.16)

add_library(core 
crud.c
hashmap.c
index.c
parser.c
schema.c
)

target_include_directories(core PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../headers)

