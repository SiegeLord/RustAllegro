cmake_minimum_required(VERSION 2.8)
project(RustAllegro NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)

set(ALLEGRO_ROOT src/allegro/lib.rs)
set(ALLEGRO_IMAGE_ROOT src/allegro_image/lib.rs)
set(EXAMPLE_ROOT examples/example.rs)
file(GLOB EXAMPLE_DATA_FILES RELATIVE "${CMAKE_SOURCE_DIR}" "${CMAKE_SOURCE_DIR}/examples/data/*")

set(RUSTC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")
set(RUSTDOC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")

# Get the dependencies of all the crates
get_rust_deps(${ALLEGRO_ROOT} ALLEGRO_DEPS)
get_rust_deps(${ALLEGRO_IMAGE_ROOT} ALLEGRO_IMAGE_DEPS)
get_rust_deps(${EXAMPLE_ROOT} EXAMPLE_DEPS)

# Build the library
rust_crate(${ALLEGRO_ROOT}
           TARGET_NAME ALLEGRO
           DESTINATION lib
           DEPENDS "${ALLEGRO_DEPS}")

rust_crate(${ALLEGRO_IMAGE_ROOT}
           TARGET_NAME ALLEGRO_IMAGE
           DESTINATION lib
           DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_IMAGE_DEPS}")

add_custom_target(library_target
                  ALL
                  DEPENDS ${ALLEGRO_FULL_TARGET} ${ALLEGRO_IMAGE_FULL_TARGET};)

# Build examples
rust_crate(${EXAMPLE_ROOT}
           TARGET_NAME EXAMPLE
           DESTINATION examples
           DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_IMAGE_FULL_TARGET};${EXAMPLE_DEPS}")

# Copy the data directory
set(EXAMPLE_DATA_FILES_DEST "")
foreach(rel_file IN ITEMS ${EXAMPLE_DATA_FILES})
   set(src "${CMAKE_SOURCE_DIR}/${rel_file}")
   set(dest "${CMAKE_BINARY_DIR}/${rel_file}")
   add_custom_command(OUTPUT "${dest}"
                      DEPENDS "${src}"
                      COMMAND "${CMAKE_COMMAND}" -E copy "${src}" "${dest}")

   list(APPEND EXAMPLE_DATA_FILES_DEST "${dest}")
endforeach()


add_custom_target(examples_target
                  ALL
                  DEPENDS ${EXAMPLE_FULL_TARGET} ${EXAMPLE_DATA_FILES_DEST})


# Build documentation
rust_doc(${ALLEGRO_ROOT}
         TARGET_NAME ALLEGRO_DOC
         DESTINATION doc
         DEPENDS "${ALLEGRO_DEPS}")

rust_doc(${ALLEGRO_IMAGE_ROOT}
         TARGET_NAME ALLEGRO_IMAGE_DOC
         DESTINATION doc
         DEPENDS "${ALLEGRO_IMAGE_DEPS}")

add_custom_target(doc
                  DEPENDS ${ALLEGRO_DOC_FULL_TARGET} ${ALLEGRO_IMAGE_DOC_FULL_TARGET})

# Install library
install(FILES ${ALLEGRO_ARTIFACTS}
        DESTINATION lib)
