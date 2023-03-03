
cmake_minimum_required (VERSION 3.7)


set(DEVELOPMENT_PROJECT_NAME "ProtocolFix")                     # <== Set to your project name, e.g. project.xcodeproj
set(DEVELOPMENT_TEAM_ID "doom.fish")    
set(FRAMEWORK_NAME "ProtocolFix")                                # <== Set to your framework's name
set(FRAMEWORK_BUNDLE_IDENTIFIER "fish.doom") 
set(DEPLOYMENT_TARGET 8.0)                                  # <== Set your deployment target version of iOS
set(DEVICE_FAMILY "1")       

project(${DEVELOPMENT_PROJECT_NAME})

set(LIBRARY_SOURCE
  ${CMAKE_CURRENT_LIST_DIR}/ProtocolFix.h
    ${CMAKE_CURRENT_LIST_DIR}/ProtocolFix.m
)

add_library(
  ${FRAMEWORK_NAME} STATIC
    ${LIBRARY_SOURCE}
)

find_library(SCREENCAPTUREKIT ScreenCaptureKit)
find_library(FOUNDATION Foundation)


target_link_libraries(${FRAMEWORK_NAME} ${SCREENCAPTUREKIT})
target_link_libraries(${FRAMEWORK_NAME} ${FOUNDATION})

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fobjc-arc -fPID")

#set_target_properties(${FRAMEWORK_NAME} PROPERTIES
#    FRAMEWORK TRUE
#    FRAMEWORK_VERSION A
#    MACOSX_FRAMEWORK_IDENTIFIER ${FRAMEWORK_BUNDLE_IDENTIFIER}
#    MACOSX_FRAMEWORK_INFO_PLIST ${CMAKE_CURRENT_LIST_DIR}/framework.plist.in
#    # "current version" in semantic format in Mach-O binary file
#    VERSION 1.0.0
#    # "compatibility version" in semantic format in Mach-O binary file
#    SOVERSION 1.0.0
#    PUBLIC_HEADER "${CMAKE_CURRENT_LIST_DIR}/ProtocolFix.h"
#    XCODE_ATTRIBUTE_IPHONEOS_DEPLOYMENT_TARGET ${DEPLOYMENT_TARGET}
#    XCODE_ATTRIBUTE_DEVELOPMENT_TEAM ${DEVELOPMENT_TEAM_ID}
#    XCODE_ATTRIBUTE_TARGETED_DEVICE_FAMILY ${DEVICE_FAMILY}
#    XCODE_ATTRIBUTE_SKIP_INSTALL "YES"
#)
#
#
#add_custom_command(
#    TARGET ${FRAMEWORK_NAME}
#    POST_BUILD
#    COMMAND /bin/bash -c "${CMAKE_CURRENT_LIST_DIR}/install_name.sh .${BUILT_PRODUCTS_DIR}/${FRAMEWORK_NAME}.framework/${FRAMEWORK_NAME}"
#)
#
# add_custom_command(
#     TARGET ${FRAMEWORK_NAME}
#     POST_BUILD
#     COMMAND install_name_tool -id \"@rpath./${FRAMEWORK_NAME}.framework/${FRAMEWORK_NAME}\"
#     .${BUILT_PRODUCTS_DIR}/${FRAMEWORK_NAME}.framework/${FRAMEWORK_NAME}
# )
