cmake_minimum_required(VERSION 3.16)

project(zkkodb VERSION 1.0
LANGUAGES C 
DESCRIPTION "zkkodb is a lightweight, file-based, SQL-inspired relational database system implemented in C. It is designed from scratch to handle structured data with features like schema validation, referential integrity, transaction logic, and persistent storage – all without relying on external database engines. The goal is to build a fully functioning database system, learning and implementing the core mechanisms manually, including indexing and parsing."
)

set(CMAKE_C_STANDARD 99)

add_subdirectory(src/core)
add_subdirectory(src/utils)

add_executable(zkkodb src/main.c)
target_link_libraries(zkkodb PRIVATE core utils)


