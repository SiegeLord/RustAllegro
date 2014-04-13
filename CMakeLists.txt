cmake_minimum_required(VERSION 2.8)
project(RustAllegro NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)

set(ALLEGRO_ROOT src/allegro/lib.rs)
set(ALLEGRO_IMAGE_ROOT src/allegro_image/lib.rs)
set(ALLEGRO_FONT_ROOT src/allegro_font/lib.rs)
set(ALLEGRO_TTF_ROOT src/allegro_ttf/lib.rs)
set(ALLEGRO_AUDIO_ROOT src/allegro_audio/lib.rs)

set(EXAMPLE_ROOT examples/example.rs)
set(MULTIPLE_WINDOWS_ROOT examples/multiple_windows.rs)

file(GLOB EXAMPLE_DATA_FILES RELATIVE "${CMAKE_SOURCE_DIR}" "${CMAKE_SOURCE_DIR}/examples/data/*")
set(CUSTOM_CSS_SRC "${CMAKE_SOURCE_DIR}/doc/main.css")
set(CUSTOM_CSS_DEST "${CMAKE_BINARY_DIR}/doc/main.css")

set(RUSTC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")
set(RUSTDOC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")

# Get the dependencies of all the crates
get_rust_deps(${ALLEGRO_ROOT} ALLEGRO_DEPS
              COMPILE
              DESTINATION lib
              OTHER_RUSTC_FLAGS --crate-type rlib)
get_rust_deps(${ALLEGRO_IMAGE_ROOT} ALLEGRO_IMAGE_DEPS)
get_rust_deps(${ALLEGRO_FONT_ROOT} ALLEGRO_FONT_DEPS)
get_rust_deps(${ALLEGRO_TTF_ROOT} ALLEGRO_TTF_DEPS)
get_rust_deps(${ALLEGRO_AUDIO_ROOT} ALLEGRO_AUDIO_DEPS)
get_rust_deps(${EXAMPLE_ROOT} EXAMPLE_DEPS)
get_rust_deps(${MULTIPLE_WINDOWS_ROOT} MULTIPLE_WINDOWS_DEPS)

# Build the library
rust_crate(${ALLEGRO_ROOT}
           TARGET_NAME ALLEGRO
           DESTINATION lib
           DEPENDS "${ALLEGRO_DEPS}"
           OTHER_RUSTC_FLAGS --crate-type rlib)

rust_crate(${ALLEGRO_IMAGE_ROOT}
           TARGET_NAME ALLEGRO_IMAGE
           DESTINATION lib
           DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_IMAGE_DEPS}"
           OTHER_RUSTC_FLAGS --crate-type rlib)

rust_crate(${ALLEGRO_FONT_ROOT}
           TARGET_NAME ALLEGRO_FONT
           DESTINATION lib
           DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_FONT_DEPS}"
           OTHER_RUSTC_FLAGS --crate-type rlib)

rust_crate(${ALLEGRO_TTF_ROOT}
           TARGET_NAME ALLEGRO_TTF
           DESTINATION lib
           DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_FONT_FULL_TARGET};${ALLEGRO_TTF_DEPS}"
           OTHER_RUSTC_FLAGS --crate-type rlib)

rust_crate(${ALLEGRO_AUDIO_ROOT}
           TARGET_NAME ALLEGRO_AUDIO
           DESTINATION lib
           DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_AUDIO_DEPS}"
           OTHER_RUSTC_FLAGS --crate-type rlib)

add_custom_target(library_target
                  ALL
                  DEPENDS ${ALLEGRO_FULL_TARGET} ${ALLEGRO_IMAGE_FULL_TARGET} ${ALLEGRO_FONT_FULL_TARGET} ${ALLEGRO_TTF_FULL_TARGET} ${ALLEGRO_AUDIO_FULL_TARGET})

# Build examples
rust_crate(${EXAMPLE_ROOT}
           TARGET_NAME EXAMPLE
           DESTINATION examples
           DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_IMAGE_FULL_TARGET};${ALLEGRO_FONT_FULL_TARGET};${ALLEGRO_TTF_FULL_TARGET};${ALLEGRO_AUDIO_FULL_TARGET};${EXAMPLE_DEPS}")

rust_crate(${MULTIPLE_WINDOWS_ROOT}
           TARGET_NAME MULTIPLE_WINDOWS
           DESTINATION examples
           DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_FONT_FULL_TARGET};${MULTIPLE_WINDOWS_DEPS}")

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
                  DEPENDS ${EXAMPLE_FULL_TARGET} ${MULTIPLE_WINDOWS_FULL_TARGET} ${EXAMPLE_DATA_FILES_DEST})


# Build documentation
rust_doc(${ALLEGRO_ROOT}
         TARGET_NAME ALLEGRO_DOC
         DESTINATION doc
         DEPENDS "${ALLEGRO_DEPS}")

rust_doc(${ALLEGRO_IMAGE_ROOT}
         TARGET_NAME ALLEGRO_IMAGE_DOC
         DESTINATION doc
         DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_IMAGE_DEPS}")

rust_doc(${ALLEGRO_FONT_ROOT}
         TARGET_NAME ALLEGRO_FONT_DOC
         DESTINATION doc
         DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_FONT_DEPS}")

rust_doc(${ALLEGRO_TTF_ROOT}
         TARGET_NAME ALLEGRO_TTF_DOC
         DESTINATION doc
         DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_TTF_DEPS}")

rust_doc(${ALLEGRO_AUDIO_ROOT}
         TARGET_NAME ALLEGRO_AUDIO_DOC
         DESTINATION doc
         DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_AUDIO_DEPS}")

# Copy the custom CSS
file(RELATIVE_PATH rel_dest "${CMAKE_BINARY_DIR}" "${CUSTOM_CSS_DEST}")
add_custom_target("DELETE_STOCK_CSS"
                  DEPENDS ${ALLEGRO_DOC_FULL_TARGET}
                  DEPENDS ${ALLEGRO_IMAGE_DOC_FULL_TARGET}
                  DEPENDS ${ALLEGRO_FONT_DOC_FULL_TARGET}
                  DEPENDS ${ALLEGRO_TTF_DOC_FULL_TARGET}
                  DEPENDS ${ALLEGRO_AUDIO_DOC_FULL_TARGET}
                  COMMAND "${CMAKE_COMMAND}" -E remove "${CUSTOM_CSS_DEST}"
                  COMMENT "Deleting stock ${rel_dest}")

add_custom_command(OUTPUT "${CUSTOM_CSS_DEST}"
                   DEPENDS "DELETE_STOCK_CSS"
                   DEPENDS "${CUSTOM_CSS_SRC}"
                   COMMAND "${CMAKE_COMMAND}" -E copy "${CUSTOM_CSS_SRC}" "${CUSTOM_CSS_DEST}")

add_custom_target(doc
                  DEPENDS ${ALLEGRO_DOC_FULL_TARGET}
                  DEPENDS ${ALLEGRO_IMAGE_DOC_FULL_TARGET}
                  DEPENDS ${ALLEGRO_FONT_DOC_FULL_TARGET}
                  DEPENDS ${ALLEGRO_TTF_DOC_FULL_TARGET}
                  DEPENDS ${ALLEGRO_AUDIO_DOC_FULL_TARGET}
                  DEPENDS ${CUSTOM_CSS_DEST})

# Install library
install(FILES ${ALLEGRO_ARTIFACTS} ${ALLEGRO_IMAGE_ARTIFACTS} ${ALLEGRO_FONT_ARTIFACTS} ${ALLEGRO_TTF_ARTIFACTS} ${ALLEGRO_AUDIO_ARTIFACTS}
        DESTINATION lib)
