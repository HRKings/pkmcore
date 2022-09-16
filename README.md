# PKMCore
---
<!--TOC-->
- [1 - Compatibility with PKHex.Core](#1---compatibility-with-pkhexcore)
- [2 - Motivation](#2---motivation)
    - [2.1 - Isn't C# portable enough?](#21---isnt-c-portable-enough)
    - [2.2 - Why low-level?](#22---why-low-level)
    - [2.3 - Why Rust?](#23---why-rust)
<!--TOC-->

This repository is a complete core library to read, write and manipulate Pokemon data and save files, as well provide some tools for researching and data restoration. It's meant to be a low level library that can be used in any applications, embedded systems, cross-platform and in a lightweight manner.

# 1 - Compatibility with PKHex.Core
This library aims provides full compatibility with [PKHex Core](https://github.com/kwsch/PKHeX), its file formats and provide all of its features up to commit [038722ee0](https://github.com/kwsch/PKHeX/tree/038722ee09fa1139b6492056e9ca359d2712f4d2)

# 2 - Motivation

Currently PKHex is written in C# (one of my favorite languages). While .NET Core is cross-platform, it was basically meant for windows, this means that something don't work out of the box, like an easy to use cross-platform GUI library, and currently the GUI of PKHex is made in Windows Forms, a platform that doesn't support Linux or MacOS.

My main OS is Linux what made me frustrated when I tried to use the app with Mono/Wine. I also wanted to make new apps that use the PKHex.Core, but I didn't want to make them in C# because of portability.

On top of that I wanted to expand and build upon the core without interfering with the philosophy of the PKHex team.

All of that made me start creating this library.

### 2.1 - Isn't C# portable enough?

While C# is cross-platform, it uses more memory than a low-level API and when made to be run from C header files, need to include the .NET runtime which itself is pretty large.

### 2.2 - Why low-level?

Because in this manner, we can have a lightweight library with minimal overhead and practically no runtime, and by using the C ABI as lingua franca of FFIs, we can call anything from anything. [Go](https://pkg.go.dev/cmd/cgo), [C#](https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/interop/interoperability-overview), C/C++ (obviously), [Python](https://docs.python.org/3/library/ctypes.html) and even [JavaScript](https://github.com/node-ffi/node-ffi), all support calling FFI and this makes this library extremely portable. So you can create applications in other languages, platforms and everything in between. Having basically no runtime means that you could also call it as a secondary process using any IPC method, and little to no overhead from the library itself.

### 2.3 - Why Rust?

When building a low-level library usually we use C or C++, but Rust offers about the same performance with benefits included, like high-level APIs during development, memory safety and a compiler that is really great, improving DX (Developer Experience).

Also we have nice frameworks like [Tauri](https://tauri.app/) which enable making beautiful cross-platform applications with relatively ease. If the library is already in Rust, there will be no overhead of FFI or IPC calling.

---
