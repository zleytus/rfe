using Rfe.Net;

Console.WriteLine($"RF Explorer driver installed: {RfExplorer.IsDriverInstalled}");

var ports = RfExplorer.PortNames();
Console.WriteLine("Ports:");
foreach (var port in ports)
{
    Console.WriteLine($"  {port}");
}

using var spectrumAnalyzer = SpectrumAnalyzer.Connect();
if (spectrumAnalyzer is not null)
{
    PrintSpectrumAnalyzerInfo(spectrumAnalyzer);
}

using var signalGenerator = SignalGenerator.Connect();
if (signalGenerator is not null)
{
    PrintSignalGeneratorInfo(signalGenerator);
}

static void PrintSpectrumAnalyzerInfo(SpectrumAnalyzer rfe)
{
    Console.WriteLine($"Spectrum Analyzer ({rfe.PortName}):");
    Console.WriteLine($"\tFirmware version: {rfe.FirmwareVersion}");
    Console.WriteLine($"\tSerial number: {rfe.SerialNumber ?? ""}");
    Console.WriteLine($"\tCenter: {rfe.CenterFrequencyHz} Hz");
    Console.WriteLine($"\tSpan: {rfe.SpanHz} Hz");
    Console.WriteLine($"\tStart: {rfe.StartFrequencyHz} Hz");
    Console.WriteLine($"\tStop: {rfe.StopFrequencyHz} Hz");
    Console.WriteLine($"\tStep: {rfe.StepSizeHz} Hz");
    Console.WriteLine($"\tRBW: {rfe.ResolutionBandwidthHz} Hz");
    Console.WriteLine($"\tSweep points: {rfe.SweepLength}");
    Console.WriteLine($"\tAmp offset: {rfe.AmplitudeOffsetDb} dB");
    Console.WriteLine($"\tMode: {rfe.Mode}");
    Console.WriteLine($"\tCalc mode: {rfe.CalcMode}");
    Console.WriteLine($"\tMin freq: {rfe.MinimumFrequencyHz} Hz");
    Console.WriteLine($"\tMax freq: {rfe.MaximumFrequencyHz} Hz");
    Console.WriteLine($"\tMax span: {rfe.MaximumSpanHz} Hz");
    Console.WriteLine($"\tMin amp: {rfe.MinimumAmplitudeDbm} dBm");
    Console.WriteLine($"\tMax amp: {rfe.MaximumAmplitudeDbm} dBm");
    Console.WriteLine($"\tActive radio module model: {SpectrumAnalyzer.GetModelName(rfe.ActiveRadioModel)}");
    Console.WriteLine($"\tInactive radio module model: {SpectrumAnalyzer.GetModelName(rfe.InactiveRadioModel)}");
    Console.WriteLine();
}

static void PrintSignalGeneratorInfo(SignalGenerator rfe)
{
    Console.WriteLine($"Signal Generator ({rfe.PortName}):");
    Console.WriteLine($"\tFirmware version: {rfe.FirmwareVersion}");
    Console.WriteLine($"\tSerial number: {rfe.SerialNumber ?? ""}");

    var config = rfe.GetConfig();
    if (config is not null)
    {
        Console.WriteLine($"\tStart: {config.Value.StartHz} Hz");
        Console.WriteLine($"\tCW: {config.Value.CwHz} Hz");
        Console.WriteLine($"\tTotal steps: {config.Value.TotalSteps}");
        Console.WriteLine($"\tStep: {config.Value.StepHz} Hz");
        Console.WriteLine($"\tAttenuation: {config.Value.Attenuation}");
        Console.WriteLine($"\tPower level: {config.Value.PowerLevel}");
        Console.WriteLine($"\tSweep power steps: {config.Value.SweepPowerSteps}");
        Console.WriteLine($"\tStart attenuation: {config.Value.StartAttenuation}");
        Console.WriteLine($"\tStart power level: {config.Value.StartPowerLevel}");
        Console.WriteLine($"\tStop attenuation: {config.Value.StopAttenuation}");
        Console.WriteLine($"\tStop power level: {config.Value.StopPowerLevel}");
        Console.WriteLine($"\tRF power: {config.Value.RfPower}");
        Console.WriteLine($"\tSweep delay: {config.Value.SweepDelayMs} ms");
    }

    Console.WriteLine();
}
