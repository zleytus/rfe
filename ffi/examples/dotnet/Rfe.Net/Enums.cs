namespace Rfe.Net;

/// <summary>RF output attenuation state.</summary>
public enum Attenuation : byte
{
    /// <summary>Attenuation is enabled.</summary>
    On = 0,
    /// <summary>Attenuation is disabled.</summary>
    Off = 1,
}

/// <summary>Discrete RF output power level.</summary>
public enum PowerLevel : byte
{
    /// <summary>Lowest output power.</summary>
    Lowest = 0,
    /// <summary>Low output power.</summary>
    Low = 1,
    /// <summary>High output power.</summary>
    High = 2,
    /// <summary>Highest output power.</summary>
    Highest = 3,
}

/// <summary>RF output power state.</summary>
public enum RfPower : byte
{
    /// <summary>RF output is enabled.</summary>
    On = 0,
    /// <summary>RF output is disabled.</summary>
    Off = 1,
}

/// <summary>Temperature range reported by the signal generator.</summary>
public enum Temperature : byte
{
    /// <summary>Temperature is between -10 C and 0 C.</summary>
    MinusTenToZero = 48,
    /// <summary>Temperature is between 0 C and 10 C.</summary>
    ZeroToTen = 49,
    /// <summary>Temperature is between 10 C and 20 C.</summary>
    TenToTwenty = 50,
    /// <summary>Temperature is between 20 C and 30 C.</summary>
    TwentyToThirty = 51,
    /// <summary>Temperature is between 30 C and 40 C.</summary>
    ThirtyToForty = 52,
    /// <summary>Temperature is between 40 C and 50 C.</summary>
    FortyToFifty = 53,
    /// <summary>Temperature is between 50 C and 60 C.</summary>
    FiftyToSixty = 54,
}

/// <summary>Operating mode reported by an RF Explorer device.</summary>
public enum Mode : byte
{
    /// <summary>Spectrum analyzer mode.</summary>
    SpectrumAnalyzer = 0,
    /// <summary>RF generator mode.</summary>
    RfGenerator = 1,
    /// <summary>Wi-Fi analyzer mode.</summary>
    WifiAnalyzer = 2,
    /// <summary>Analyzer tracking mode.</summary>
    AnalyzerTracking = 5,
    /// <summary>RF sniffer mode.</summary>
    RfSniffer = 6,
    /// <summary>CW transmitter mode.</summary>
    CwTransmitter = 60,
    /// <summary>Frequency sweep mode.</summary>
    SweepFrequency = 61,
    /// <summary>Amplitude sweep mode.</summary>
    SweepAmplitude = 62,
    /// <summary>Generator tracking mode.</summary>
    GeneratorTracking = 63,
    /// <summary>Unknown or unsupported mode.</summary>
    Unknown = 255,
}

/// <summary>Sweep calculator mode used by the spectrum analyzer.</summary>
public enum CalcMode : byte
{
    /// <summary>Normal sweep display.</summary>
    Normal = 0,
    /// <summary>Maximum value mode.</summary>
    Max = 1,
    /// <summary>Average value mode.</summary>
    Avg = 2,
    /// <summary>Overwrite mode.</summary>
    Overwrite = 3,
    /// <summary>Maximum hold mode.</summary>
    MaxHold = 4,
    /// <summary>Historical maximum mode.</summary>
    MaxHistorical = 5,
    /// <summary>Unknown or unsupported calculator mode.</summary>
    Unknown = 255,
}

/// <summary>Digital signal processing mode used by the spectrum analyzer.</summary>
public enum DspMode : byte
{
    /// <summary>Automatically select the DSP mode.</summary>
    Auto = 0,
    /// <summary>Filtered DSP mode.</summary>
    Filter = 1,
    /// <summary>Fast DSP mode.</summary>
    Fast = 2,
    /// <summary>No image rejection DSP mode.</summary>
    NoImg = 3,
}

/// <summary>RF input stage selected on supported spectrum analyzer models.</summary>
public enum InputStage : byte
{
    /// <summary>Direct input path.</summary>
    Direct = 48,
    /// <summary>30 dB attenuator input path.</summary>
    Attenuator30dB = 49,
    /// <summary>25 dB low-noise amplifier input path.</summary>
    Lna25dB = 50,
    /// <summary>60 dB attenuator input path.</summary>
    Attenuator60dB = 51,
    /// <summary>12 dB low-noise amplifier input path.</summary>
    Lna12dB = 52,
}

/// <summary>Status of analyzer tracking mode.</summary>
public enum TrackingStatus : byte
{
    /// <summary>Tracking mode is disabled.</summary>
    Disabled = 0,
    /// <summary>Tracking mode is enabled.</summary>
    Enabled = 1,
}

/// <summary>Wi-Fi band used by Wi-Fi analyzer mode.</summary>
public enum WifiBand : byte
{
    /// <summary>2.4 GHz Wi-Fi band.</summary>
    TwoPointFourGhz = 1,
    /// <summary>5 GHz Wi-Fi band.</summary>
    FiveGhz = 2,
}

/// <summary>Signal generator model reported by the RF Explorer.</summary>
public enum SignalGeneratorModel : byte
{
    /// <summary>Main 6 GHz signal generator module.</summary>
    Rfe6Gen = 60,
    /// <summary>Expansion 6 GHz signal generator module.</summary>
    Rfe6GenExpansion = 61,
}

/// <summary>RF Explorer spectrum analyzer model.</summary>
public enum SpectrumAnalyzerModel : byte
{
    /// <summary>433M model.</summary>
    Rfe433M = 0,
    /// <summary>868M model.</summary>
    Rfe868M = 1,
    /// <summary>915M model.</summary>
    Rfe915M = 2,
    /// <summary>WSUB1G model.</summary>
    RfeWSub1G = 3,
    /// <summary>2.4G model.</summary>
    Rfe24G = 4,
    /// <summary>WSUB3G model.</summary>
    RfeWSub3G = 5,
    /// <summary>6G model.</summary>
    Rfe6G = 6,
    /// <summary>WSUB1G+ model.</summary>
    RfeWSub1GPlus = 10,
    /// <summary>Pro Audio model.</summary>
    RfeProAudio = 11,
    /// <summary>2.4G+ model.</summary>
    Rfe24GPlus = 12,
    /// <summary>4G+ model.</summary>
    Rfe4GPlus = 13,
    /// <summary>6G+ model.</summary>
    Rfe6GPlus = 14,
    /// <summary>MW5G 3 GHz model.</summary>
    RfeMW5G3G = 16,
    /// <summary>MW5G 4 GHz model.</summary>
    RfeMW5G4G = 17,
    /// <summary>MW5G 5 GHz model.</summary>
    RfeMW5G5G = 18,
    /// <summary>Unknown or unsupported model.</summary>
    Unknown = 19,
}
