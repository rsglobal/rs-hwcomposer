/*
 * SPDX-License-Identifier: Apache-2.0
 */

#include "libui_stub.h"
#include "plane_layouts.h"

namespace android {

template <>
GraphicBufferMapper
    *android::Singleton<android::GraphicBufferMapper>::sInstance = nullptr;

EXPORT status_t GraphicBufferMapper::importBufferNoValidate(
    const native_handle_t *rawHandle, buffer_handle_t *outHandle) {
  return 0;
}

EXPORT status_t GraphicBufferMapper::freeBuffer(buffer_handle_t handle) {
  return 0;
}

EXPORT status_t GraphicBufferMapper::getWidth(buffer_handle_t bufferHandle,
                                              uint64_t *outWidth) {
  return 0;
}
EXPORT status_t GraphicBufferMapper::getHeight(buffer_handle_t bufferHandle,
                                               uint64_t *outHeight) {
  return 0;
}

EXPORT status_t GraphicBufferMapper::getPixelFormatFourCC(
    buffer_handle_t bufferHandle, uint32_t *outPixelFormatFourCC) {
  return 0;
}

EXPORT status_t GraphicBufferMapper::getPixelFormatModifier(
    buffer_handle_t bufferHandle, uint64_t *outPixelFormatModifier) {
  return 0;
}

// EXPORT status_t GraphicBufferMapper::getPlaneLayouts(
//     buffer_handle_t bufferHandle,
//     vector<PlaneLayout, std::allocator<PlaneLayout>> *outPlaneLayouts) {}

GraphicBufferMapper::GraphicBufferMapper() {}

}; // namespace android

extern "C" {
void GET_PLANE_LAYOUTS_MANGLED() {}
}
