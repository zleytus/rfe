# Rfe.Net

A .NET wrapper for RF Explorer spectrum analyzers and signal generators.

## Requirements

- [.NET 10 SDK](https://dotnet.microsoft.com/download)
- [Rust toolchain](https://rustup.rs) to build the native library

## Build

First, build the native rfe library from the project root:

```sh
cargo build --release -p rfe-ffi
```

Then build the .NET solution:

```sh
cd ffi/examples/dotnet
dotnet build
```

Run the sweep example:

```sh
dotnet run --project examples/Sweep
```

## Usage

```csharp
using Rfe.Net;

var portNames = RfExplorer.PortNames();

using var analyzer = SpectrumAnalyzer.Connect();
if (analyzer is not null)
{
    Console.WriteLine(analyzer.PortName);
    Console.WriteLine(analyzer.FirmwareVersion);
}
```
