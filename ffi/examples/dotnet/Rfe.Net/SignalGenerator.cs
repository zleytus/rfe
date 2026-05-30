using CsBindgen;

namespace Rfe.Net;

/// <summary>An RF Explorer signal generator. Dispose this instance when done.</summary>
public sealed class SignalGenerator : IDisposable
{
    private readonly unsafe CsBindgen.SignalGenerator* _ptr;
    private bool _disposed;

    private unsafe SignalGenerator(CsBindgen.SignalGenerator* ptr)
    {
        _ptr = ptr;
    }

    /// <summary>Connects to the first RF Explorer signal generator found on a CP210x USB serial port.</summary>
    public static SignalGenerator? Connect()
    {
        unsafe
        {
            var ptr = NativeMethods.rfe_signal_generator_connect();
            return ptr == null ? null : new SignalGenerator(ptr);
        }
    }

    /// <summary>Connects to a named serial port using the given baud rate.</summary>
    public static SignalGenerator? Connect(string portName, uint baudRate)
    {
        unsafe
        {
            var name = NativeHelpers.ToNullTerminatedUtf8(portName);
            fixed (byte* namePtr = name)
            {
                var ptr = NativeMethods.rfe_signal_generator_connect_with_name_and_baud_rate(namePtr, baudRate);
                return ptr == null ? null : new SignalGenerator(ptr);
            }
        }
    }

    /// <summary>Returns the display name for a signal generator model.</summary>
    public static string GetModelName(SignalGeneratorModel model)
    {
        unsafe
        {
            return NativeHelpers.ReadString(
                () => 100,
                (buffer, length) => NativeMethods.rfe_signal_generator_model_name(
                    (CsBindgen.SignalGeneratorModel)model,
                    buffer,
                    length));
        }
    }

    /// <summary>The connected serial port name.</summary>
    public string PortName
    {
        get
        {
            unsafe
            {
                return NativeHelpers.ReadString(
                    () => NativeMethods.rfe_signal_generator_port_name_len(_ptr),
                    (buffer, length) => NativeMethods.rfe_signal_generator_port_name(_ptr, buffer, length));
            }
        }
    }

    /// <summary>The firmware version reported by the device.</summary>
    public string FirmwareVersion
    {
        get
        {
            unsafe
            {
                return NativeHelpers.ReadString(
                    () => NativeMethods.rfe_signal_generator_firmware_version_len(_ptr),
                    (buffer, length) => NativeMethods.rfe_signal_generator_firmware_version(_ptr, buffer, length));
            }
        }
    }

    /// <summary>The serial number reported by the device, or null if unavailable.</summary>
    public string? SerialNumber
    {
        get
        {
            unsafe
            {
                return NativeHelpers.ReadOptionalString(
                    () => NativeMethods.rfe_signal_generator_serial_number_len(_ptr),
                    (buffer, length) => NativeMethods.rfe_signal_generator_serial_number(_ptr, buffer, length));
            }
        }
    }

    /// <summary>The most recent main signal generator configuration, or null if unavailable.</summary>
    public SignalGeneratorConfig? GetConfig()
    {
        unsafe
        {
            var config = new CsBindgen.SignalGeneratorConfig();
            var result = NativeMethods.rfe_signal_generator_config(_ptr, &config);
            return result == CsBindgen.Result.NoData ? null : ReturnValue(result, SignalGeneratorConfig.FromNative(config));
        }
    }

    /// <summary>The most recent amplitude sweep configuration, or null if unavailable.</summary>
    public SignalGeneratorConfigAmpSweep? GetConfigAmpSweep()
    {
        unsafe
        {
            var config = new CsBindgen.SignalGeneratorConfigAmpSweep();
            var result = NativeMethods.rfe_signal_generator_config_amp_sweep(_ptr, &config);
            return result == CsBindgen.Result.NoData ? null : ReturnValue(result, SignalGeneratorConfigAmpSweep.FromNative(config));
        }
    }

    /// <summary>The most recent CW configuration, or null if unavailable.</summary>
    public SignalGeneratorConfigCw? GetConfigCw()
    {
        unsafe
        {
            var config = new CsBindgen.SignalGeneratorConfigCw();
            var result = NativeMethods.rfe_signal_generator_config_cw(_ptr, &config);
            return result == CsBindgen.Result.NoData ? null : ReturnValue(result, SignalGeneratorConfigCw.FromNative(config));
        }
    }

    /// <summary>The most recent frequency sweep configuration, or null if unavailable.</summary>
    public SignalGeneratorConfigFreqSweep? GetConfigFreqSweep()
    {
        unsafe
        {
            var config = new CsBindgen.SignalGeneratorConfigFreqSweep();
            var result = NativeMethods.rfe_signal_generator_config_freq_sweep(_ptr, &config);
            return result == CsBindgen.Result.NoData ? null : ReturnValue(result, SignalGeneratorConfigFreqSweep.FromNative(config));
        }
    }

    /// <summary>The most recent temperature range, or null if unavailable.</summary>
    public Temperature? GetTemperature()
    {
        unsafe
        {
            var temperature = new CsBindgen.Temperature();
            var result = NativeMethods.rfe_signal_generator_temperature(_ptr, &temperature);
            return result == CsBindgen.Result.NoData ? null : ReturnValue(result, (Temperature)temperature);
        }
    }

    /// <summary>The main radio module model, or null if unavailable.</summary>
    public SignalGeneratorModel? GetMainRadioModel()
    {
        unsafe
        {
            var model = new CsBindgen.SignalGeneratorModel();
            var result = NativeMethods.rfe_signal_generator_main_radio_model(_ptr, &model);
            return result == CsBindgen.Result.NoData ? null : ReturnValue(result, (SignalGeneratorModel)model);
        }
    }

    /// <summary>The expansion radio module model, or null if unavailable.</summary>
    public SignalGeneratorModel? GetExpansionRadioModel()
    {
        unsafe
        {
            var model = new CsBindgen.SignalGeneratorModel();
            var result = NativeMethods.rfe_signal_generator_expansion_radio_model(_ptr, &model);
            return result == CsBindgen.Result.NoData ? null : ReturnValue(result, (SignalGeneratorModel)model);
        }
    }

    /// <summary>The currently active radio module model.</summary>
    public SignalGeneratorModel ActiveRadioModel
    {
        get
        {
            unsafe
            {
                var model = new CsBindgen.SignalGeneratorModel();
                RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_active_radio_model(_ptr, &model));
                return (SignalGeneratorModel)model;
            }
        }
    }

    /// <summary>The currently inactive radio module model, or null if unavailable.</summary>
    public SignalGeneratorModel? GetInactiveRadioModel()
    {
        unsafe
        {
            var model = new CsBindgen.SignalGeneratorModel();
            var result = NativeMethods.rfe_signal_generator_inactive_radio_model(_ptr, &model);
            return result == CsBindgen.Result.NoData ? null : ReturnValue(result, (SignalGeneratorModel)model);
        }
    }

    /// <summary>Turns the signal generator LCD on.</summary>
    public void LcdOn()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_lcd_on(_ptr));
        }
    }

    /// <summary>Turns the signal generator LCD off.</summary>
    public void LcdOff()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_lcd_off(_ptr));
        }
    }

    /// <summary>Enables screen dump messages from the signal generator.</summary>
    public void EnableDumpScreen()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_enable_dump_screen(_ptr));
        }
    }

    /// <summary>Disables screen dump messages from the signal generator.</summary>
    public void DisableDumpScreen()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_disable_dump_screen(_ptr));
        }
    }

    /// <summary>Holds the current signal generator operation.</summary>
    public void Hold()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_hold(_ptr));
        }
    }

    /// <summary>Reboots the signal generator.</summary>
    public void Reboot()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_reboot(_ptr));
        }
    }

    /// <summary>Powers off the signal generator.</summary>
    public void PowerOff()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_power_off(_ptr));
        }
    }

    /// <summary>Sends raw bytes to the signal generator.</summary>
    public void SendBytes(ReadOnlySpan<byte> bytes)
    {
        unsafe
        {
            fixed (byte* ptr = bytes)
            {
                RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_send_bytes(_ptr, ptr, (nuint)bytes.Length));
            }
        }
    }

    /// <summary>Returns the most recent LCD screen capture, or null if unavailable.</summary>
    public ScreenData? ScreenData()
    {
        unsafe
        {
            CsBindgen.ScreenData* screenData = null;
            var result = NativeMethods.rfe_signal_generator_screen_data(_ptr, &screenData);
            return result == CsBindgen.Result.NoData ? null : FromScreenResult(result, screenData);
        }
    }

    /// <summary>Waits for the next LCD screen capture.</summary>
    public ScreenData WaitForNextScreenData()
    {
        unsafe
        {
            CsBindgen.ScreenData* screenData = null;
            var result = NativeMethods.rfe_signal_generator_wait_for_next_screen_data(_ptr, &screenData);
            return FromScreenResult(result, screenData);
        }
    }

    /// <summary>Waits up to the given timeout for the next LCD screen capture.</summary>
    public ScreenData WaitForNextScreenData(TimeSpan timeout)
    {
        unsafe
        {
            CsBindgen.ScreenData* screenData = null;
            var result = NativeMethods.rfe_signal_generator_wait_for_next_screen_data_with_timeout(_ptr, (ulong)timeout.TotalSeconds, &screenData);
            return FromScreenResult(result, screenData);
        }
    }

    /// <summary>Starts amplitude sweep mode.</summary>
    public void StartAmpSweep(ulong cwHz, Attenuation startAttenuation, PowerLevel startPowerLevel, Attenuation stopAttenuation, PowerLevel stopPowerLevel, byte stepDelaySeconds)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_start_amp_sweep(_ptr, cwHz, (CsBindgen.Attenuation)startAttenuation, (CsBindgen.PowerLevel)startPowerLevel, (CsBindgen.Attenuation)stopAttenuation, (CsBindgen.PowerLevel)stopPowerLevel, stepDelaySeconds));
        }
    }

    /// <summary>Starts amplitude sweep mode using the expansion module.</summary>
    public void StartAmpSweepExp(ulong cwHz, double startPowerDbm, double stepPowerDb, double stopPowerDbm, byte stepDelaySeconds)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_start_amp_sweep_exp(_ptr, cwHz, startPowerDbm, stepPowerDb, stopPowerDbm, stepDelaySeconds));
        }
    }

    /// <summary>Starts CW mode.</summary>
    public void StartCw(ulong cwHz, Attenuation attenuation, PowerLevel powerLevel)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_start_cw(_ptr, cwHz, (CsBindgen.Attenuation)attenuation, (CsBindgen.PowerLevel)powerLevel));
        }
    }

    /// <summary>Starts CW mode using the expansion module.</summary>
    public void StartCwExp(ulong cwHz, double powerDbm)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_start_cw_exp(_ptr, cwHz, powerDbm));
        }
    }

    /// <summary>Starts frequency sweep mode.</summary>
    public void StartFreqSweep(ulong startHz, Attenuation attenuation, PowerLevel powerLevel, ushort sweepSteps, ulong stepHz, byte stepDelaySeconds)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_start_freq_sweep(_ptr, startHz, (CsBindgen.Attenuation)attenuation, (CsBindgen.PowerLevel)powerLevel, sweepSteps, stepHz, stepDelaySeconds));
        }
    }

    /// <summary>Starts frequency sweep mode using the expansion module.</summary>
    public void StartFreqSweepExp(ulong startHz, double powerDbm, ushort sweepSteps, ulong stepHz, byte stepDelaySeconds)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_start_freq_sweep_exp(_ptr, startHz, powerDbm, sweepSteps, stepHz, stepDelaySeconds));
        }
    }

    /// <summary>Starts tracking mode.</summary>
    public void StartTracking(ulong startHz, Attenuation attenuation, PowerLevel powerLevel, ushort sweepSteps, ulong stepHz)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_start_tracking(_ptr, startHz, (CsBindgen.Attenuation)attenuation, (CsBindgen.PowerLevel)powerLevel, sweepSteps, stepHz));
        }
    }

    /// <summary>Starts tracking mode using the expansion module.</summary>
    public void StartTrackingExp(ulong startHz, double powerDbm, ushort sweepSteps, ulong stepHz)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_start_tracking_exp(_ptr, startHz, powerDbm, sweepSteps, stepHz));
        }
    }

    /// <summary>Jumps to a new frequency using the tracking step frequency.</summary>
    public void TrackingStep(ushort steps)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_tracking_step(_ptr, steps));
        }
    }

    /// <summary>Turns RF output power on.</summary>
    public void RfPowerOn()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_rf_power_on(_ptr));
        }
    }

    /// <summary>Turns RF output power off.</summary>
    public void RfPowerOff()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_signal_generator_rf_power_off(_ptr));
        }
    }

    /// <inheritdoc/>
    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        _disposed = true;
        unsafe
        {
            NativeMethods.rfe_signal_generator_free(_ptr);
        }

        GC.SuppressFinalize(this);
    }

    /// <inheritdoc/>
    ~SignalGenerator()
    {
        Dispose();
    }

    private static T ReturnValue<T>(CsBindgen.Result result, T value)
    {
        RfeException.ThrowIfError(result);
        return value;
    }

    private static unsafe ScreenData FromScreenResult(CsBindgen.Result result, CsBindgen.ScreenData* screenData)
    {
        RfeException.ThrowIfError(result);
        return new ScreenData(screenData);
    }
}
