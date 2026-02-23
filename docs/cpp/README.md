# eStream Marketplace — C++ Guide

## Installation

Add to your `CMakeLists.txt`:

```cmake
find_package(estream REQUIRED COMPONENTS marketplace)
target_link_libraries(my_target PRIVATE estream::marketplace)
```

Or fetch directly via CMake FetchContent:

```cmake
include(FetchContent)
FetchContent_Declare(
  estream
  GIT_REPOSITORY https://github.com/polyquantum/estream-cpp.git
  GIT_TAG v0.9.1
)
FetchContent_MakeAvailable(estream)
target_link_libraries(my_target PRIVATE estream::marketplace)
```

## Consuming Components

### Browsing the Marketplace

```cpp
#include <estream/marketplace.h>
#include <iostream>

int main() {
    auto client = estream::marketplace::Client();

    auto components = client.search({
        .query = "wire-adapter",
        .category = estream::marketplace::Category::WireAdapter,
        .limit = 20,
    });

    for (const auto& comp : components) {
        std::cout << comp.name() << " v" << comp.version()
                  << " — " << comp.description() << "\n";
    }
}
```

### Installing a Component

```cpp
auto comp = client.install("estream-wire-fix", "^1.0.0");
std::cout << "Installed " << comp.name() << " at " << comp.install_path() << "\n";
```

### Using a Component

Once installed, the component headers and libraries are available via CMake:

```cpp
#include <estream/wire/fix.h>

estream::wire::FixDecoder decoder;
auto msg = decoder.decode(raw_data, raw_len);
std::cout << "Order type: " << msg.ord_type() << "\n";
```

### Checking Component Lifecycle

```cpp
#include <estream/marketplace.h>

auto comp = client.get("estream-wire-fix", "^1.0.0");

switch (comp.lifecycle().status()) {
    case estream::marketplace::Lifecycle::Active:
        std::cout << "Component is active\n";
        break;
    case estream::marketplace::Lifecycle::Deprecated:
        std::cout << "Deprecated — migrate to " << comp.lifecycle().successor()
                  << " before " << comp.lifecycle().sunset() << "\n";
        break;
    case estream::marketplace::Lifecycle::Sunset:
        std::cout << "Component has been sunset and is no longer supported\n";
        break;
}

// Run automated migration check across your project
auto report = client.lifecycle_audit();
for (const auto& warning : report.deprecated()) {
    std::cerr << "WARN: " << warning.name()
              << " is deprecated, successor: " << warning.successor() << "\n";
}
```

## Publishing Components

### Prerequisites

- eStream CLI installed (`estream --version` >= 0.9.1)
- Marketplace account (`estream marketplace login`)

### Create a Component

```bash
estream marketplace scaffold smart-circuit my-circuit --lang cpp
cd my-circuit
```

This generates the project structure with `CMakeLists.txt`, `estream-component.toml`, headers, source files, and tests.

### Define the Manifest

`estream-component.toml`:

```toml
[component]
name = "my-circuit"
version = "0.1.0"
description = "Custom SmartCircuit for order validation"
category = "smart-circuit"
license = "Apache-2.0"
authors = ["Your Name <you@example.com>"]

[component.implementation]
type = "cpp"
entry = "include/my_circuit/circuit.h"
min_estream = "0.9.1"
std = "c++20"

[component.lifecycle]
status = "draft"
```

### Build and Test

```bash
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build
ctest --test-dir build
estream marketplace validate .
```

The `validate` command checks manifest completeness, schema conformance, and runs lint passes.

### Publish

```bash
estream marketplace publish --release
```

### Version Updates

```bash
# Update version in estream-component.toml, then:

# Deprecate the old version with a successor pointer
estream marketplace lifecycle set 1.0.0 deprecated \
  --successor "my-circuit@^1.1.0" \
  --sunset "2027-06-01"

# Publish the new version
estream marketplace publish --release
```
