/*
 * SPDX-License-Identifier: Apache-2.0
 */

#pragma once

#include <cstdint>
#include <string>
#include <vector>

using ::std::string;
using ::std::vector;

#ifdef __cplusplus
extern "C" {
#endif

typedef struct native_handle {
  int version;
  int numFds;
  int numInts;
  int data[0];
} native_handle_t;

typedef const native_handle_t *buffer_handle_t;

#ifdef __cplusplus
}
#endif

namespace aidl::android::hardware::graphics::common {

class ExtendableType {
public:
  ~ExtendableType() {}
  string name;
  int64_t value = 0L;
};

class PlaneLayoutComponent {
public:
  ExtendableType type;
  int64_t offsetInBits = 0L;
  int64_t sizeInBits = 0L;
};

class PlaneLayout {
public:
  vector<PlaneLayoutComponent> components;
  int64_t offsetInBytes = 0L;
  int64_t sampleIncrementInBits = 0L;
  int64_t strideInBytes = 0L;
  int64_t widthInSamples = 0L;
  int64_t heightInSamples = 0L;
  int64_t totalSizeInBytes = 0L;
  int64_t horizontalSubsampling = 0L;
  int64_t verticalSubsampling = 0L;
};

} // namespace aidl::android::hardware::graphics::common
