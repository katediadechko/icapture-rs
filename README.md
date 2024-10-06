# icapture-rs

`icapture-rs` is a client-server application for capturing images and video on Windows. It uses Microsoft Media Foundation (unsafe) to detect available capturing devices and OpenCV Rust bindings to communicate with the selected device.

## Build

1. [Install](https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-cmd#1---set-up-vcpkg) `vcpkg` package manager.

   ```PowerShell
   git clone https://github.com/microsoft/vcpkg.git
   cd vcpkg && bootstrap-vcpkg.bat
   ```

2. Specify the path to the `vcpkg` repository as the following environment variable and add it to the path.

   ```PowerShell
   $env:VCPKG_ROOT="C:\Git\vcpkg"
   $env:PATH += $env:VCPKG_ROOT + ";"
   ```

3. [Install](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md#windows-package) `llvm` and `opencv` packages (be patient - it took two hours on my laptop).

   ```PowerShell
   vcpkg install llvm opencv4[contrib,nonfree]
   ```

4. Add `clang` directory to the `%PATH%`.

   ```PowerShell
   $env:PATH += $env:VCPKG_ROOT + "\packages\llvm_x64-windows\tools\llvm" + ";"
   ```

5. Set the following environment variable.

   ```PowerShell
   $env:VCPKGRS_DYNAMIC=1
   ```

6. Build with `cargo build -vv`.

## Run

### Command Line

1. icapture-rs uses [env_logger](https://docs.rs/env_logger/latest/env_logger/index.html) for logging and tracing. Set the `RUST_LOG` environment variable to manage the logging level and scope. For instance,

   ```PowerShell
   $env:RUST_LOG="debug"
   ```

2. Make sure the configuration file `config.json` is located in the current directory. Update the configuration if necessary. If the file is not present or invalid, default capturing parameters will be used.

3. Run with `icapture_cli.exe`.

### REST Server

1. Run with `cargo run` or `icapture_srv.exe`.

2. The available endpoints are the following.

   ```
   GET http://localhost:1212/list      # list available capturing devices
   POST http://localhost:1212/init     # initialize capturing
   POST http://localhost:1212/preview  # preview the capturing stream
   POST http://localhost:1212/frame    # grab the current frame
   POST http://localhost:1212/start    # start grabbing frames
   POST http://localhost:1212/stop     # stop grabbing frames
   POST http://localhost:1212/deinit   # de-initialize capturing
   ```

   The `init` endpoint requires a request body containing capturing configuration, see the `config.json` file for reference. If the body is empty or its content is invalid, default capturing parameters will be used.

## Test

1. Few existing unit tests can be run with `cargo test`.

2. The solution has been developed and tested on Windows 10, version 22H2 with [Magewell USB Capture HDMI 4K Plus](https://www.magewell.com/products/usb-capture-hdmi-4k-plus) card.
