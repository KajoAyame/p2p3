# Install script for directory: /Users/Signal/.cargo/registry/src/github.com-88ac128001ac3a9a/libssh2-sys-0.1.37/libssh2

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/Signal/Rust_Project/p2p3/target/debug/build/libssh2-sys-5c335ae487a3a8b6/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

if(NOT CMAKE_INSTALL_COMPONENT OR "${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified")
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/doc/libssh2" TYPE FILE FILES
    "/Users/Signal/.cargo/registry/src/github.com-88ac128001ac3a9a/libssh2-sys-0.1.37/libssh2/docs/AUTHORS"
    "/Users/Signal/.cargo/registry/src/github.com-88ac128001ac3a9a/libssh2-sys-0.1.37/libssh2/COPYING"
    "/Users/Signal/.cargo/registry/src/github.com-88ac128001ac3a9a/libssh2-sys-0.1.37/libssh2/docs/HACKING"
    "/Users/Signal/.cargo/registry/src/github.com-88ac128001ac3a9a/libssh2-sys-0.1.37/libssh2/README"
    "/Users/Signal/.cargo/registry/src/github.com-88ac128001ac3a9a/libssh2-sys-0.1.37/libssh2/RELEASE-NOTES"
    "/Users/Signal/.cargo/registry/src/github.com-88ac128001ac3a9a/libssh2-sys-0.1.37/libssh2/NEWS"
    )
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/Users/Signal/Rust_Project/p2p3/target/debug/build/libssh2-sys-5c335ae487a3a8b6/out/build/src/cmake_install.cmake")
  include("/Users/Signal/Rust_Project/p2p3/target/debug/build/libssh2-sys-5c335ae487a3a8b6/out/build/docs/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/Users/Signal/Rust_Project/p2p3/target/debug/build/libssh2-sys-5c335ae487a3a8b6/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
