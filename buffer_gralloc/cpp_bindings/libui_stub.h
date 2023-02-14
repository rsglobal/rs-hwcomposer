/*
 * SPDX-License-Identifier: Apache-2.0
 */

#pragma once

#include "plane_layouts.h"
#include <memory>
#include <stdint.h>

using aidl::android::hardware::graphics::common::PlaneLayout;

// ndk mangled name does not match the name of the libui.so built within AOSP. So
// we need to define the mangled name here instead of relying on the compiler.
#define GET_PLANE_LAYOUTS_MANGLED                                              \
  _ZN7android19GraphicBufferMapper15getPlaneLayoutsEPK13native_handlePNSt3__16vectorIN4aidl7android8hardware8graphics6common11PlaneLayoutENS4_9allocatorISB_EEEE

#define EXPORT __attribute__((visibility("default")))

namespace android {

template <typename TYPE> class Singleton {
public:
  static TYPE &getInstance() {
    TYPE *instance = sInstance;
    if (instance == nullptr) {
      instance = new TYPE();
      sInstance = instance;
    }
    return *instance;
  }

  static bool hasInstance();

protected:
  ~Singleton() {}
  Singleton() {}

private:
  Singleton(const Singleton &);
  Singleton &operator=(const Singleton &);

  static TYPE *sInstance;
};

typedef int32_t status_t;

class GrallocMapper {};

class GraphicBufferMapper : public Singleton<GraphicBufferMapper> {
public:
  enum Version {
    GRALLOC_2 = 2,
    GRALLOC_3,
    GRALLOC_4,
    GRALLOC_5,
  };
  status_t importBufferNoValidate(const native_handle_t *rawHandle,
                                  buffer_handle_t *outHandle);

  status_t freeBuffer(buffer_handle_t handle);

  status_t getWidth(buffer_handle_t bufferHandle, uint64_t *outWidth);
  status_t getHeight(buffer_handle_t bufferHandle, uint64_t *outHeight);

  status_t getPixelFormatFourCC(buffer_handle_t bufferHandle,
                                uint32_t *outPixelFormatFourCC);
  status_t getPixelFormatModifier(buffer_handle_t bufferHandle,
                                  uint64_t *outPixelFormatModifier);

  status_t getPlaneLayouts(buffer_handle_t bufferHandle,
                           vector<PlaneLayout> *outPlaneLayouts);

private:
  friend class Singleton<GraphicBufferMapper>;

  GraphicBufferMapper();

  std::unique_ptr<const GrallocMapper> mMapper;

  Version mMapperVersion;
};

}; // namespace android
