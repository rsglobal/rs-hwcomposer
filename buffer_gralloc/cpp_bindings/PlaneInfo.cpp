#include <cstdint>
#include <cstring>

#include "libui_stub.h"
#include "plane_layouts.h"

using aidl::android::hardware::graphics::common::PlaneLayout;

class BufferHandle {
public:
  buffer_handle_t handle{};
  vector<PlaneLayout> planes;

  ~BufferHandle() {
    if (handle) {
      android::GraphicBufferMapper::getInstance().freeBuffer(handle);
    }
  }
};

extern "C" {

android::status_t
GET_PLANE_LAYOUTS_MANGLED(android::GraphicBufferMapper *self,
                          buffer_handle_t bufferHandle,
                          vector<PlaneLayout> *outPlaneLayouts);

auto gralloc_import_buffer(int32_t num_fds, int32_t num_ints, const int *fds,
                           const int *ints) -> BufferHandle * {
  auto size =
      sizeof(native_handle_t) + num_fds * sizeof(int) + num_ints * sizeof(int);
  auto rawHandle = (native_handle_t *)malloc(size);
  rawHandle->version = sizeof(native_handle_t);
  rawHandle->numFds = num_fds;
  rawHandle->numInts = num_ints;
  memcpy(rawHandle->data, fds, num_fds * sizeof(int));
  memcpy(rawHandle->data + num_fds, ints, num_ints * sizeof(int));

  android::GraphicBufferMapper &inst =
      android::GraphicBufferMapper::getInstance();

  auto outHandle = new BufferHandle();
  auto status = inst.importBufferNoValidate(rawHandle, &outHandle->handle);
  free(rawHandle);

  if (status != 0) {
    delete outHandle;
    return nullptr;
  }

  status =
      GET_PLANE_LAYOUTS_MANGLED(&inst, outHandle->handle, &outHandle->planes);
  if (status != 0) {
    delete outHandle;
    return nullptr;
  }

  return outHandle;
}

void gralloc_free_buffer(BufferHandle *bufferHandle) { delete bufferHandle; }

auto gralloc_get_plane_count(BufferHandle *bufferHandle) -> size_t {
  return bufferHandle->planes.size();
}

auto gralloc_get_plane_param(BufferHandle *bufferHandle, size_t planeIndex,
                             int64_t *offsetInBytes, int64_t *strideInBytes)
    -> bool {
  if (planeIndex >= bufferHandle->planes.size()) {
    return false;
  }
  *offsetInBytes = bufferHandle->planes[planeIndex].offsetInBytes;
  *strideInBytes = bufferHandle->planes[planeIndex].strideInBytes;
  return true;
}

auto gralloc_get_format_modifier(BufferHandle *bufferHandle,
                                 uint32_t *outFormat,
                                 uint64_t *outFormatModifier) -> bool {
  auto status =
      android::GraphicBufferMapper::getInstance().getPixelFormatFourCC(
          bufferHandle->handle, outFormat);
  if (status != 0) {
    return false;
  }

  status = android::GraphicBufferMapper::getInstance().getPixelFormatModifier(
      bufferHandle->handle, outFormatModifier);

  return status == 0;
}

auto gralloc_get_width_height(BufferHandle *bufferHandle, uint64_t *outWidth,
                              uint64_t *outHeight) -> bool {
  auto status = android::GraphicBufferMapper::getInstance().getWidth(
      bufferHandle->handle, outWidth);
  if (status != 0) {
    return false;
  }

  status = android::GraphicBufferMapper::getInstance().getHeight(
      bufferHandle->handle, outHeight);

  return status == 0;
}

auto gralloc_get_fds(BufferHandle *bufferHandle, int *fds_count,
                     const int **fds) -> bool {
  if (bufferHandle->handle->numFds == 0) {
    return false;
  }
  *fds_count = bufferHandle->handle->numFds;
  *fds = bufferHandle->handle->data;
  return true;
}
}
