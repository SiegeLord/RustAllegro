cmake_minimum_required(VERSION 2.8)
project(RustAllegro NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)

option(USE_LINK_NAME "Whether to use #[link(name=lib] directives. Turn off if you want to set link_args manually." on)

set(ALLEGRO_ROOT src/allegro/lib.rs)

set(EXAMPLE_ROOT examples/example.rs)
set(MULTIPLE_WINDOWS_ROOT examples/multiple_windows.rs)
set(AUDIO_EXAMPLE_ROOT examples/audio_example.rs)

file(GLOB EXAMPLE_DATA_FILES RELATIVE "${CMAKE_SOURCE_DIR}" "${CMAKE_SOURCE_DIR}/examples/data/*")
set(CUSTOM_CSS_SRC "${CMAKE_SOURCE_DIR}/doc/main.css")
set(CUSTOM_CSS_DEST "${CMAKE_BINARY_DIR}/doc/main.css")

set(RUSTC_FLAGS -L ${CMAKE_BINARY_DIR}/lib)
set(RUSTDOC_FLAGS -L ${CMAKE_BINARY_DIR}/lib)

if(USE_LINK_NAME)
   list(APPEND RUSTC_FLAGS --cfg use_link_name)
endif()

set(LIB_TARGET_DEPENDS "")
set(DOC_TARGET_DEPENDS "")
set(ARTIFACTS "")

macro(add_addon addon_root addon_name other_deps)
   get_rust_deps(${addon_root} ${addon_name}_DEPS)

   rust_crate(${addon_root}
           TARGET_NAME ${addon_name}
           DESTINATION lib
           DEPENDS "${ALLEGRO_FULL_TARGET};${${addon_name}_DEPS};${other_deps}"
           OTHER_RUSTC_FLAGS --crate-type rlib)

   list(APPEND LIB_TARGET_DEPENDS "${${addon_name}_FULL_TARGET}")
   list(APPEND ARTIFACTS "${${addon_name}_ARTIFACTS}")

   rust_doc(${addon_root}
         TARGET_NAME ${addon_name}_DOC
         DESTINATION doc
         DEPENDS "${ALLEGRO_FULL_TARGET};${${addon_name}_DEPS};${other_deps}")

   list(APPEND DOC_TARGET_DEPENDS "${${addon_name}_DOC_FULL_TARGET}")
endmacro()

# Get the dependencies of all the crates
get_rust_deps(${ALLEGRO_ROOT} ALLEGRO_DEPS
              COMPILE
              DESTINATION lib
              OTHER_RUSTC_FLAGS --crate-type rlib)

get_rust_deps(${EXAMPLE_ROOT} EXAMPLE_DEPS)
get_rust_deps(${MULTIPLE_WINDOWS_ROOT} MULTIPLE_WINDOWS_DEPS)
get_rust_deps(${AUDIO_EXAMPLE_ROOT} AUDIO_EXAMPLE_DEPS)

# Build the library
rust_crate(${ALLEGRO_ROOT}
           TARGET_NAME ALLEGRO
           DESTINATION lib
           DEPENDS "${ALLEGRO_DEPS}"
           OTHER_RUSTC_FLAGS --crate-type rlib)

add_addon(src/allegro_image/lib.rs ALLEGRO_IMAGE "")
add_addon(src/allegro_font/lib.rs ALLEGRO_FONT "")
add_addon(src/allegro_ttf/lib.rs ALLEGRO_TTF "${ALLEGRO_FONT_FULL_TARGET}")
add_addon(src/allegro_audio/lib.rs ALLEGRO_AUDIO "")
add_addon(src/allegro_acodec/lib.rs ALLEGRO_ACODEC "${ALLEGRO_AUDIO_FULL_TARGET}")
add_addon(src/allegro_primitives/lib.rs ALLEGRO_PRIMITIVES "")
add_addon(src/allegro_dialog/lib.rs ALLEGRO_DIALOG "")

add_custom_target(library_target
                  ALL
                  DEPENDS ${LIB_TARGET_DEPENDS})

# Build examples
rust_crate(${EXAMPLE_ROOT}
           TARGET_NAME EXAMPLE
           DESTINATION examples
           DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_IMAGE_FULL_TARGET};${ALLEGRO_FONT_FULL_TARGET};${ALLEGRO_TTF_FULL_TARGET};${ALLEGRO_PRIMITIVES_FULL_TARGET};${EXAMPLE_DEPS}"
           OTHER_RUSTC_FLAGS -g)

rust_crate(${MULTIPLE_WINDOWS_ROOT}
           TARGET_NAME MULTIPLE_WINDOWS
           DESTINATION examples
           DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_FONT_FULL_TARGET};${MULTIPLE_WINDOWS_DEPS}"
           OTHER_RUSTC_FLAGS -g)

rust_crate(${AUDIO_EXAMPLE_ROOT}
           TARGET_NAME AUDIO_EXAMPLE
           DESTINATION examples
           DEPENDS "${ALLEGRO_FULL_TARGET};${ALLEGRO_FONT_FULL_TARGET};${ALLEGRO_AUDIO_FULL_TARGET};${ALLEGRO_ACODEC_FULL_TARGET};${ALLEGRO_PRIMITIVES_FULL_TARGET};${AUDIO_EXAMPLE_DEPS}"
           OTHER_RUSTC_FLAGS -g)

# Build documentation

rust_doc(${ALLEGRO_ROOT}
         TARGET_NAME ALLEGRO_DOC
         DESTINATION doc
         DEPENDS "${ALLEGRO_DEPS}")


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
                  DEPENDS ${EXAMPLE_FULL_TARGET} ${AUDIO_EXAMPLE_FULL_TARGET} ${MULTIPLE_WINDOWS_FULL_TARGET} ${EXAMPLE_DATA_FILES_DEST})

# Copy the custom CSS
file(RELATIVE_PATH rel_dest "${CMAKE_BINARY_DIR}" "${CUSTOM_CSS_DEST}")
add_custom_target("DELETE_STOCK_CSS"
                  DEPENDS ${ALLEGRO_DOC_FULL_TARGET}
                  DEPENDS ${DOC_TARGET_DEPENDS}
                  COMMAND "${CMAKE_COMMAND}" -E remove "${CUSTOM_CSS_DEST}"
                  COMMENT "Deleting stock ${rel_dest}")

add_custom_command(OUTPUT "${CUSTOM_CSS_DEST}"
                   DEPENDS "DELETE_STOCK_CSS"
                   DEPENDS "${CUSTOM_CSS_SRC}"
                   COMMAND "${CMAKE_COMMAND}" -E copy "${CUSTOM_CSS_SRC}" "${CUSTOM_CSS_DEST}")

add_custom_target(doc
                  DEPENDS ${ALLEGRO_DOC_FULL_TARGET}
                  DEPENDS ${DOC_TARGET_DEPENDS}
                  DEPENDS ${CUSTOM_CSS_DEST})

# Install library
install(FILES ${ALLEGRO_ARTIFACTS} ${ARTIFACTS}
        DESTINATION lib)
