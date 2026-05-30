// This file is NOT auto-generated and should not be modified by build tools.
//
// csbindgen generates P/Invoke bindings for the rfe FFI crate, but some pointer
// and enum types in the FFI signatures are defined by the Rust rfe library
// rather than by rfe-ffi. These declarations complete the generated bindings.

using System.Runtime.InteropServices;

namespace CsBindgen
{
    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct SignalGenerator { }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct SpectrumAnalyzer { }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct ScreenData { }

    internal enum Attenuation : byte
    {
        On = 0,
        Off = 1,
    }

    internal enum PowerLevel : byte
    {
        Lowest = 0,
        Low = 1,
        High = 2,
        Highest = 3,
    }

    internal enum RfPower : byte
    {
        On = 0,
        Off = 1,
    }

    internal enum Temperature : byte
    {
        MinusTenToZero = 48,
        ZeroToTen = 49,
        TenToTwenty = 50,
        TwentyToThirty = 51,
        ThirtyToForty = 52,
        FortyToFifty = 53,
        FiftyToSixty = 54,
    }

    internal enum Mode : byte
    {
        SpectrumAnalyzer = 0,
        RfGenerator = 1,
        WifiAnalyzer = 2,
        AnalyzerTracking = 5,
        RfSniffer = 6,
        CwTransmitter = 60,
        SweepFrequency = 61,
        SweepAmplitude = 62,
        GeneratorTracking = 63,
        Unknown = 255,
    }

    internal enum CalcMode : byte
    {
        Normal = 0,
        Max = 1,
        Avg = 2,
        Overwrite = 3,
        MaxHold = 4,
        MaxHistorical = 5,
        Unknown = 255,
    }

    internal enum DspMode : byte
    {
        Auto = 0,
        Filter = 1,
        Fast = 2,
        NoImg = 3,
    }

    internal enum InputStage : byte
    {
        Direct = 48,
        Attenuator30dB = 49,
        Lna25dB = 50,
        Attenuator60dB = 51,
        Lna12dB = 52,
    }

    internal enum TrackingStatus : byte
    {
        Disabled = 0,
        Enabled = 1,
    }

    internal enum WifiBand : byte
    {
        TwoPointFourGhz = 1,
        FiveGhz = 2,
    }
}
