# icapture-rs

## Build

### Windows

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

4. Add `clang` directory to the `%PATH%.

   ```
   set PATH=%PATH%;C:\Git\vcpkg\packages\llvm_x64-windows\tools\llvm
   ```

5. Restart the IDE (Visual Studio Code or other) to reload environment variables.

6. Build with `cargo build`.
