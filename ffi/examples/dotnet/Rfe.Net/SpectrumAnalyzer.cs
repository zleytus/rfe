using CsBindgen;

namespace Rfe.Net;

/// <summary>An RF Explorer spectrum analyzer. Dispose this instance when done.</summary>
public sealed class SpectrumAnalyzer : IDisposable
{
    private readonly unsafe CsBindgen.SpectrumAnalyzer* _ptr;
    private bool _disposed;

    private unsafe SpectrumAnalyzer(CsBindgen.SpectrumAnalyzer* ptr)
    {
        _ptr = ptr;
    }

    /// <summary>Connects to the first RF Explorer spectrum analyzer found on a CP210x USB serial port.</summary>
    public static SpectrumAnalyzer? Connect()
    {
        unsafe
        {
            var ptr = NativeMethods.rfe_spectrum_analyzer_connect();
            return ptr == null ? null : new SpectrumAnalyzer(ptr);
        }
    }

    /// <summary>Connects to a named serial port using the given baud rate.</summary>
    public static SpectrumAnalyzer? Connect(string portName, uint baudRate)
    {
        unsafe
        {
            var name = NativeHelpers.ToNullTerminatedUtf8(portName);
            fixed (byte* namePtr = name)
            {
                var ptr = NativeMethods.rfe_spectrum_analyzer_connect_with_name_and_baud_rate(namePtr, baudRate);
                return ptr == null ? null : new SpectrumAnalyzer(ptr);
            }
        }
    }

    /// <summary>Returns the display name for a spectrum analyzer model.</summary>
    public static string GetModelName(SpectrumAnalyzerModel model)
    {
        unsafe
        {
            return NativeHelpers.ReadString(
                () => 100,
                (buffer, length) => NativeMethods.rfe_spectrum_analyzer_model_name(
                    (CsBindgen.SpectrumAnalyzerModel)model,
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
                    () => NativeMethods.rfe_spectrum_analyzer_port_name_len(_ptr),
                    (buffer, length) => NativeMethods.rfe_spectrum_analyzer_port_name(_ptr, buffer, length));
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
                    () => NativeMethods.rfe_spectrum_analyzer_firmware_version_len(_ptr),
                    (buffer, length) => NativeMethods.rfe_spectrum_analyzer_firmware_version(_ptr, buffer, length));
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
                    () => NativeMethods.rfe_spectrum_analyzer_serial_number_len(_ptr),
                    (buffer, length) => NativeMethods.rfe_spectrum_analyzer_serial_number(_ptr, buffer, length));
            }
        }
    }

    /// <summary>The current sweep start frequency in hertz.</summary>
    public ulong StartFrequencyHz
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_start_freq_hz(_ptr);
            }
        }
    }

    /// <summary>The current sweep step size in hertz.</summary>
    public ulong StepSizeHz
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_step_size_hz(_ptr);
            }
        }
    }

    /// <summary>The current sweep stop frequency in hertz.</summary>
    public ulong StopFrequencyHz
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_stop_freq_hz(_ptr);
            }
        }
    }

    /// <summary>The current sweep center frequency in hertz.</summary>
    public ulong CenterFrequencyHz
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_center_freq_hz(_ptr);
            }
        }
    }

    /// <summary>The current sweep span in hertz.</summary>
    public ulong SpanHz
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_span_hz(_ptr);
            }
        }
    }

    /// <summary>The active radio module's minimum supported frequency in hertz.</summary>
    public ulong MinimumFrequencyHz
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_min_freq_hz(_ptr);
            }
        }
    }

    /// <summary>The active radio module's maximum supported frequency in hertz.</summary>
    public ulong MaximumFrequencyHz
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_max_freq_hz(_ptr);
            }
        }
    }

    /// <summary>The active radio module's maximum supported span in hertz.</summary>
    public ulong MaximumSpanHz
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_max_span_hz(_ptr);
            }
        }
    }

    /// <summary>The resolution bandwidth in hertz, or zero if unavailable.</summary>
    public ulong ResolutionBandwidthHz
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_rbw_hz(_ptr);
            }
        }
    }

    /// <summary>The bottom displayed amplitude in dBm.</summary>
    public short MinimumAmplitudeDbm
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_min_amp_dbm(_ptr);
            }
        }
    }

    /// <summary>The top displayed amplitude in dBm.</summary>
    public short MaximumAmplitudeDbm
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_max_amp_dbm(_ptr);
            }
        }
    }

    /// <summary>The amplitude offset in dB, or zero if unavailable.</summary>
    public sbyte AmplitudeOffsetDb
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_amp_offset_db(_ptr);
            }
        }
    }

    /// <summary>The number of points in each sweep.</summary>
    public ushort SweepLength
    {
        get
        {
            unsafe
            {
                return NativeMethods.rfe_spectrum_analyzer_sweep_len(_ptr);
            }
        }
    }

    /// <summary>The current operating mode.</summary>
    public Mode Mode
    {
        get
        {
            unsafe
            {
                return (Mode)NativeMethods.rfe_spectrum_analyzer_mode(_ptr);
            }
        }
    }

    /// <summary>The current calculator mode.</summary>
    public CalcMode CalcMode
    {
        get
        {
            unsafe
            {
                return (CalcMode)NativeMethods.rfe_spectrum_analyzer_calc_mode(_ptr);
            }
        }
    }

    /// <summary>The main radio module model.</summary>
    public SpectrumAnalyzerModel MainRadioModel
    {
        get
        {
            unsafe
            {
                return (SpectrumAnalyzerModel)NativeMethods.rfe_spectrum_analyzer_main_radio_model(_ptr);
            }
        }
    }

    /// <summary>The expansion radio module model.</summary>
    public SpectrumAnalyzerModel ExpansionRadioModel
    {
        get
        {
            unsafe
            {
                return (SpectrumAnalyzerModel)NativeMethods.rfe_spectrum_analyzer_expansion_radio_model(_ptr);
            }
        }
    }

    /// <summary>The currently active radio module model.</summary>
    public SpectrumAnalyzerModel ActiveRadioModel
    {
        get
        {
            unsafe
            {
                return (SpectrumAnalyzerModel)NativeMethods.rfe_spectrum_analyzer_active_radio_model(_ptr);
            }
        }
    }

    /// <summary>The currently inactive radio module model.</summary>
    public SpectrumAnalyzerModel InactiveRadioModel
    {
        get
        {
            unsafe
            {
                return (SpectrumAnalyzerModel)NativeMethods.rfe_spectrum_analyzer_inactive_radio_model(_ptr);
            }
        }
    }

    /// <summary>Turns the spectrum analyzer LCD on.</summary>
    public void LcdOn()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_lcd_on(_ptr));
        }
    }

    /// <summary>Turns the spectrum analyzer LCD off.</summary>
    public void LcdOff()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_lcd_off(_ptr));
        }
    }

    /// <summary>Enables screen dump messages from the spectrum analyzer.</summary>
    public void EnableDumpScreen()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_enable_dump_screen(_ptr));
        }
    }

    /// <summary>Disables screen dump messages from the spectrum analyzer.</summary>
    public void DisableDumpScreen()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_disable_dump_screen(_ptr));
        }
    }

    /// <summary>Holds the current spectrum analyzer sweep.</summary>
    public void Hold()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_hold(_ptr));
        }
    }

    /// <summary>Reboots the spectrum analyzer.</summary>
    public void Reboot()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_reboot(_ptr));
        }
    }

    /// <summary>Powers off the spectrum analyzer.</summary>
    public void PowerOff()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_power_off(_ptr));
        }
    }

    /// <summary>Sends raw bytes to the spectrum analyzer.</summary>
    public void SendBytes(ReadOnlySpan<byte> bytes)
    {
        unsafe
        {
            fixed (byte* ptr = bytes)
            {
                RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_send_bytes(_ptr, ptr, (nuint)bytes.Length));
            }
        }
    }

    /// <summary>Copies the most recent sweep.</summary>
    public float[] Sweep()
    {
        unsafe
        {
            var sweep = new float[SweepLength];
            fixed (float* ptr = sweep)
            {
                nuint length = 0;
                RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_sweep(_ptr, ptr, (nuint)sweep.Length, &length));
                Array.Resize(ref sweep, (int)length);
                return sweep;
            }
        }
    }

    /// <summary>Waits for the next sweep.</summary>
    public float[] WaitForNextSweep()
    {
        unsafe
        {
            var sweep = new float[SweepLength];
            fixed (float* ptr = sweep)
            {
                nuint length = 0;
                RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_wait_for_next_sweep(_ptr, ptr, (nuint)sweep.Length, &length));
                Array.Resize(ref sweep, (int)length);
                return sweep;
            }
        }
    }

    /// <summary>Waits up to the given timeout for the next sweep.</summary>
    public float[] WaitForNextSweep(TimeSpan timeout)
    {
        unsafe
        {
            var sweep = new float[SweepLength];
            fixed (float* ptr = sweep)
            {
                nuint length = 0;
                RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_wait_for_next_sweep_with_timeout(
                    _ptr,
                    (ulong)timeout.TotalSeconds,
                    ptr,
                    (nuint)sweep.Length,
                    &length));
                Array.Resize(ref sweep, (int)length);
                return sweep;
            }
        }
    }

    /// <summary>Returns the most recent LCD screen capture, or null if unavailable.</summary>
    public ScreenData? ScreenData()
    {
        unsafe
        {
            CsBindgen.ScreenData* screenData = null;
            var result = NativeMethods.rfe_spectrum_analyzer_screen_data(_ptr, &screenData);
            return result == CsBindgen.Result.NoData ? null : FromScreenResult(result, screenData);
        }
    }

    /// <summary>Waits for the next LCD screen capture.</summary>
    public ScreenData WaitForNextScreenData()
    {
        unsafe
        {
            CsBindgen.ScreenData* screenData = null;
            var result = NativeMethods.rfe_spectrum_analyzer_wait_for_next_screen_data(_ptr, &screenData);
            return FromScreenResult(result, screenData);
        }
    }

    /// <summary>Waits up to the given timeout for the next LCD screen capture.</summary>
    public ScreenData WaitForNextScreenData(TimeSpan timeout)
    {
        unsafe
        {
            CsBindgen.ScreenData* screenData = null;
            var result = NativeMethods.rfe_spectrum_analyzer_wait_for_next_screen_data_with_timeout(_ptr, (ulong)timeout.TotalSeconds, &screenData);
            return FromScreenResult(result, screenData);
        }
    }

    /// <summary>The current DSP mode, or null if unavailable.</summary>
    public DspMode? GetDspMode()
    {
        unsafe
        {
            var mode = new CsBindgen.DspMode();
            var result = NativeMethods.rfe_spectrum_analyzer_dsp_mode(_ptr, &mode);
            return result == CsBindgen.Result.NoData ? null : ReturnValue(result, (DspMode)mode);
        }
    }

    /// <summary>The current tracking status, or null if unavailable.</summary>
    public TrackingStatus? GetTrackingStatus()
    {
        unsafe
        {
            var status = new CsBindgen.TrackingStatus();
            var result = NativeMethods.rfe_spectrum_analyzer_tracking_status(_ptr, &status);
            return result == CsBindgen.Result.NoData ? null : ReturnValue(result, (TrackingStatus)status);
        }
    }

    /// <summary>The current input stage, or null if unavailable.</summary>
    public InputStage? GetInputStage()
    {
        unsafe
        {
            var inputStage = new CsBindgen.InputStage();
            var result = NativeMethods.rfe_spectrum_analyzer_input_stage(_ptr, &inputStage);
            return result == CsBindgen.Result.NoData ? null : ReturnValue(result, (InputStage)inputStage);
        }
    }

    /// <summary>Starts Wi-Fi analyzer mode for the requested Wi-Fi band.</summary>
    public void StartWifiAnalyzer(WifiBand wifiBand)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_start_wifi_analyzer(_ptr, (CsBindgen.WifiBand)wifiBand));
        }
    }

    /// <summary>Stops Wi-Fi analyzer mode.</summary>
    public void StopWifiAnalyzer()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_stop_wifi_analyzer(_ptr));
        }
    }

    /// <summary>Requests tracking mode and waits for a tracking status response.</summary>
    public void RequestTracking(ulong startHz, ulong stepHz)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_request_tracking(_ptr, startHz, stepHz));
        }
    }

    /// <summary>Steps over the tracking step frequency and makes a measurement.</summary>
    public void TrackingStep(ushort step)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_tracking_step(_ptr, step));
        }
    }

    /// <summary>Sets the sweep start and stop frequencies in hertz.</summary>
    public void SetStartStop(ulong startHz, ulong stopHz)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_set_start_stop(_ptr, startHz, stopHz));
        }
    }

    /// <summary>Sets the sweep start frequency, stop frequency, and number of sweep points.</summary>
    public void SetStartStop(ulong startHz, ulong stopHz, ushort sweepLength)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_set_start_stop_sweep_len(_ptr, startHz, stopHz, sweepLength));
        }
    }

    /// <summary>Sets the sweep center frequency and span in hertz.</summary>
    public void SetCenterSpan(ulong centerHz, ulong spanHz)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_set_center_span(_ptr, centerHz, spanHz));
        }
    }

    /// <summary>Sets the sweep center frequency, span, and number of sweep points.</summary>
    public void SetCenterSpan(ulong centerHz, ulong spanHz, ushort sweepLength)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_set_center_span_sweep_len(_ptr, centerHz, spanHz, sweepLength));
        }
    }

    /// <summary>Sets the minimum and maximum amplitudes displayed on the RF Explorer screen.</summary>
    public void SetMinimumMaximumAmplitudes(short minimumDbm, short maximumDbm)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_set_min_max_amps(_ptr, minimumDbm, maximumDbm));
        }
    }

    /// <summary>Sets the number of points in each sweep.</summary>
    public void SetSweepLength(ushort sweepLength)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_set_sweep_len(_ptr, sweepLength));
        }
    }

    /// <summary>Sets the calculator mode.</summary>
    public void SetCalcMode(CalcMode calcMode)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_set_calc_mode(_ptr, (CsBindgen.CalcMode)calcMode));
        }
    }

    /// <summary>Activates the main radio module.</summary>
    public void ActivateMainRadio()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_activate_main_radio(_ptr));
        }
    }

    /// <summary>Activates the expansion radio module.</summary>
    public void ActivateExpansionRadio()
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_activate_expansion_radio(_ptr));
        }
    }

    /// <summary>Sets the spectrum analyzer input stage.</summary>
    public void SetInputStage(InputStage inputStage)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_set_input_stage(_ptr, (CsBindgen.InputStage)inputStage));
        }
    }

    /// <summary>Sets the amplitude offset in dB.</summary>
    public void SetOffsetDb(sbyte offsetDb)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_set_offset_db(_ptr, offsetDb));
        }
    }

    /// <summary>Sets the DSP mode.</summary>
    public void SetDspMode(DspMode dspMode)
    {
        unsafe
        {
            RfeException.ThrowIfError(NativeMethods.rfe_spectrum_analyzer_set_dsp_mode(_ptr, (CsBindgen.DspMode)dspMode));
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
            NativeMethods.rfe_spectrum_analyzer_free(_ptr);
        }

        GC.SuppressFinalize(this);
    }

    /// <inheritdoc/>
    ~SpectrumAnalyzer()
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
