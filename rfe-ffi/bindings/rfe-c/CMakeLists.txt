cmake_minimum_required(VERSION 3.4)

project(rfe-c LANGUAGES C)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

include_directories(../../include)

# Use Corrosion to integrate the Rust rfe library into CMake
include(FetchContent)
FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
)
FetchContent_MakeAvailable(Corrosion)
corrosion_import_crate(MANIFEST_PATH ../../Cargo.toml CRATES rfe-ffi)

# For MSVC, enable experimental support for C atomics
# See: https://devblogs.microsoft.com/cppblog/c11-atomics-in-visual-studio-2022-version-17-5-preview-2/
if(MSVC)
  add_compile_options(/experimental:c11atomics /std:c17)
endif()

set(EXAMPLES rfe_info rfe_sweep rfe_sweep_with_callback)

foreach(example ${EXAMPLES})
  add_executable(${example} ${example}.c)

  # When linking to the static version of the Rust rfe library, we need to explicitly link its dependencies
  if(BUILD_SHARED_LIBS)
    target_link_libraries(${example} PRIVATE rfe)
  elseif(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    find_package(PkgConfig REQUIRED)
    pkg_check_modules(LIBUDEV REQUIRED IMPORTED_TARGET libudev)
    target_link_libraries(${example} PRIVATE rfe PkgConfig::LIBUDEV)
  elseif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    target_link_libraries(${example} PRIVATE rfe PRIVATE SetupApi)
  elseif(CMAKE_SYSTEM_NAME STREQUAL "Darwin")
    target_link_libraries(${example} PRIVATE rfe "-framework CoreFoundation" "-framework IOKit")
  endif()
endforeach()
