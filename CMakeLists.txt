cmake_minimum_required(VERSION 3.13)
project(VTNCRW VERSION 0.1 )
#set(TARGET_NAME VTNCRW)
add_library(VTNCRW STATIC src/vtncrw.cpp src/vtncrw.h)
target_sources(${PROJECT_NAME} 
    PRIVATE 
        src/vtncrw.cpp
)
target_include_directories(${PROJECT_NAME} PUBLIC src)
set_target_properties(${PROJECT_NAME} PROPERTIES PUBLIC_HEADER src/vtncrw.h)