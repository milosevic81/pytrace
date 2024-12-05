cmake_minimum_required(VERSION 3.10)

# Set the project name
project(CppTracer)

# Specify the C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Enable code coverage flags
if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    add_compile_options(--coverage)
    add_link_options(--coverage)
endif()

# Add the Vector3 library
add_library(CppTracer src/vector.cpp)

# Add the Google Test framework
# You can download and unpack Google Test manually or use the FetchContent module
include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/release-1.11.0.zip
)
FetchContent_MakeAvailable(googletest)

# Add the test executable
add_executable(CppTracerTests test/test_vector.cpp)

# Link the test executable against Google Test and the Vector3 library
target_link_libraries(CppTracerTests gtest_main CppTracer)

# Enable testing
enable_testing()

# Add a test
add_test(NAME CppTracerTests COMMAND CppTracerTests --gtest_output=xml:report.xml)

# Add a custom target to generate the coverage report
if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    add_custom_target(coverage
        COMMAND ${CMAKE_COMMAND} -E env GCOV_PREFIX=${CMAKE_BINARY_DIR}/CMakeFiles/CMakeTmp
        COMMAND ${CMAKE_COMMAND} -E env GCOV_PREFIX_STRIP=3
        COMMAND ${CMAKE_COMMAND} -E env GCOV_PREFIX=${CMAKE_BINARY_DIR}
        COMMAND ${CMAKE_COMMAND} -E env GCOV_PREFIX_STRIP=3
        COMMAND lcov --capture --directory . --output-file coverage.info
        COMMAND lcov --remove coverage.info '/usr/*' --output-file coverage.info
        COMMAND lcov --list coverage.info
        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
    )
endif()