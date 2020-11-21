cmake_minimum_required(VERSION 3.17)
project(s57tiler)

set(CMAKE_CXX_STANDARD 20)

find_package(GDAL REQUIRED)
message("gdal found ${GDAL_FOUND}")
message("gdal include dir ${GDAL_INCLUDE_DIR}")

add_executable(s57tiler main.cpp S57.h S57.cpp S57Layer.cpp S57Layer.h)
target_include_directories(s57tiler PRIVATE ${GDAL_INCLUDE_DIR})

if(UNIX AND NOT APPLE)
    target_link_libraries(s57tiler ${GDAL_LIBRARY} stdc++fs)
else()
    target_link_libraries(s57tiler ${GDAL_LIBRARY})
endif()
