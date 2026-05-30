using CsBindgen;

namespace Rfe.Net;

/// <summary>Spectrum analyzer configuration.</summary>
public readonly record struct SpectrumAnalyzerConfig(
    ulong StartFrequencyHz,
    ulong StepSizeHz,
    ulong StopFrequencyHz,
    ulong CenterFrequencyHz,
    ulong SpanHz,
    short MaxAmplitudeDbm,
    short MinAmplitudeDbm,
    ushort SweepLength,
    bool IsExpansionRadioModuleActive,
    Mode Mode,
    ulong MinimumFrequencyHz,
    ulong MaximumFrequencyHz,
    ulong MaximumSpanHz,
    ulong ResolutionBandwidthHz,
    sbyte AmplitudeOffsetDb,
    CalcMode CalcMode)
{
    internal static SpectrumAnalyzerConfig FromNative(CsBindgen.SpectrumAnalyzerConfig config) =>
        new(
            config.start_freq_hz,
            config.step_size_hz,
            config.stop_freq_hz,
            config.center_freq_hz,
            config.span_hz,
            config.max_amp_dbm,
            config.min_amp_dbm,
            config.sweep_len,
            config.is_expansion_radio_module_active,
            (Mode)config.mode,
            config.min_freq_hz,
            config.max_freq_hz,
            config.max_span_hz,
            config.rbw_hz,
            config.amp_offset_db,
            (CalcMode)config.calc_mode);
}

/// <summary>Signal generator configuration.</summary>
public readonly record struct SignalGeneratorConfig(
    ulong StartHz,
    ulong CwHz,
    uint TotalSteps,
    ulong StepHz,
    Attenuation Attenuation,
    PowerLevel PowerLevel,
    ushort SweepPowerSteps,
    Attenuation StartAttenuation,
    PowerLevel StartPowerLevel,
    Attenuation StopAttenuation,
    PowerLevel StopPowerLevel,
    RfPower RfPower,
    ulong SweepDelayMs)
{
    internal static SignalGeneratorConfig FromNative(CsBindgen.SignalGeneratorConfig config) =>
        new(
            config.start_hz,
            config.cw_hz,
            config.total_steps,
            config.step_hz,
            (Attenuation)config.attenuation,
            (PowerLevel)config.power_level,
            config.sweep_power_steps,
            (Attenuation)config.start_attenuation,
            (PowerLevel)config.start_power_level,
            (Attenuation)config.stop_attenuation,
            (PowerLevel)config.stop_power_level,
            (RfPower)config.rf_power,
            config.sweep_delay_ms);
}

/// <summary>Signal generator amplitude sweep configuration.</summary>
public readonly record struct SignalGeneratorConfigAmpSweep(
    ulong CwHz,
    ushort SweepPowerSteps,
    Attenuation StartAttenuation,
    PowerLevel StartPowerLevel,
    Attenuation StopAttenuation,
    PowerLevel StopPowerLevel,
    RfPower RfPower,
    ulong SweepDelayMs)
{
    internal static SignalGeneratorConfigAmpSweep FromNative(CsBindgen.SignalGeneratorConfigAmpSweep config) =>
        new(
            config.cw_hz,
            config.sweep_power_steps,
            (Attenuation)config.start_attenuation,
            (PowerLevel)config.start_power_level,
            (Attenuation)config.stop_attenuation,
            (PowerLevel)config.stop_power_level,
            (RfPower)config.rf_power,
            config.sweep_delay_ms);
}

/// <summary>Signal generator CW configuration.</summary>
public readonly record struct SignalGeneratorConfigCw(
    ulong CwHz,
    uint TotalSteps,
    ulong StepFrequencyHz,
    Attenuation Attenuation,
    PowerLevel PowerLevel,
    RfPower RfPower)
{
    internal static SignalGeneratorConfigCw FromNative(CsBindgen.SignalGeneratorConfigCw config) =>
        new(
            config.cw_hz,
            config.total_steps,
            config.step_freq_hz,
            (Attenuation)config.attenuation,
            (PowerLevel)config.power_level,
            (RfPower)config.rf_power);
}

/// <summary>Signal generator frequency sweep configuration.</summary>
public readonly record struct SignalGeneratorConfigFreqSweep(
    ulong StartHz,
    uint TotalSteps,
    ulong StepHz,
    Attenuation Attenuation,
    PowerLevel PowerLevel,
    RfPower RfPower,
    ulong SweepDelayMs)
{
    internal static SignalGeneratorConfigFreqSweep FromNative(CsBindgen.SignalGeneratorConfigFreqSweep config) =>
        new(
            config.start_hz,
            config.total_steps,
            config.step_hz,
            (Attenuation)config.attenuation,
            (PowerLevel)config.power_level,
            (RfPower)config.rf_power,
            config.sweep_delay_ms);
}
