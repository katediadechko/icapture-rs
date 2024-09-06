# icapture-rs

Client-server application for capturing images and video on Windows.

## Build

1. [Install](https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-cmd#1---set-up-vcpkg) `vcpkg` package manager.

   ```
   git clone https://github.com/microsoft/vcpkg.git
   cd vcpkg && bootstrap-vcpkg.bat
   ```

2. Specify the path to the `vcpkg` repository as the following environment variable and add it to the `%PATH%`.

   ```
   set VCPKG_ROOT=C:\Git\vcpkg
   set PATH=%PATH%;%VCPKG_ROOT%
   ```

3. [Install](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md#windows-package) `llvm` and `opencv` packages (be patient - it took two hours on my laptop).

   ```
   vcpkg install llvm opencv4[contrib,nonfree]
   ```

4. Add `clang` directory to the `%PATH%`.

   ```
   set PATH=%PATH%;%VCPKG_ROOT%\packages\llvm_x64-windows\tools\llvm
   ```

5. Set the following environment variable.

   ```
   set VCPKGRS_DYNAMIC=1
   ```

6. Restart the IDE or command prompt where you build the project to reload environment variables.

7. Build with `cargo build -vv`.

## Run

1. icapture-rs uses [env_logger](https://docs.rs/env_logger/latest/env_logger/index.html) for logging and tracing. Set the `RUST_LOG` environment variable to manage the logging level and scope. For instance,

   ```
   set RUST_LOG=trace
   ```

2. Make sure the configuration file `config.json` is located in the current directory. Update the configuration if necessary. If the file is not present or invalid, default capturing parameters will be used.

3. Run with `cargo run` or `icapture_cli.exe`.

## Test

1. Few existing unit tests can be run with `cargo test`.

2. The solution has been developed and tested on Windows 10, version 22H2 with [Magewell USB Capture HDMI 4K Plus](https://www.magewell.com/products/usb-capture-hdmi-4k-plus) card.
