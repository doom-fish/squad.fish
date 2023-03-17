cmake_minimum_required (VERSION 3.7)

set(FRAMEWORK_NAME "protocol_fix")                                # <== Set to your framework's name

project("protocol_fix")

set(LIBRARY_SOURCE
  ${CMAKE_CURRENT_LIST_DIR}/ProtocolFix.h
    ${CMAKE_CURRENT_LIST_DIR}/ProtocolFix.m
)

add_library(
  ${FRAMEWORK_NAME} SHARED
    ${LIBRARY_SOURCE}
)

find_library(SCREENCAPTUREKIT ScreenCaptureKit)
find_library(FOUNDATION Foundation)


target_link_libraries(${FRAMEWORK_NAME} ${SCREENCAPTUREKIT})
target_link_libraries(${FRAMEWORK_NAME} ${FOUNDATION})

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fobjc-arc -fPID")

