file(GLOB HW_SOURCES "generated/*.c")
idf_component_register(SRCS ${HW_SOURCES}
    INCLUDE_DIRS "generated" REQUIRES esp_timer esp_hw_support esp_system spi_flash esp_driver_uart esp_driver_usb_serial_jtag)
target_compile_options(${COMPONENT_LIB} PRIVATE -O3 -fno-lto -fno-tree-vectorize)
target_link_libraries(${COMPONENT_LIB} INTERFACE "${CMAKE_CURRENT_LIST_DIR}/generated/kernels.a")
include("${CMAKE_CURRENT_LIST_DIR}/generated/baselines.cmake")
