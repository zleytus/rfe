using CsBindgen;

namespace Rfe.Net;

/// <summary>An RF Explorer LCD screen capture. Dispose this instance when done.</summary>
public sealed class ScreenData : IDisposable
{
    private readonly unsafe CsBindgen.ScreenData* _ptr;
    private bool _disposed;

    internal unsafe ScreenData(CsBindgen.ScreenData* ptr)
    {
        _ptr = ptr;
    }

    /// <summary>Gets a screen pixel. The top-left pixel is <c>(0, 0)</c>.</summary>
    public bool GetPixel(byte x, byte y)
    {
        unsafe
        {
            bool pixel = false;
            RfeException.ThrowIfError(NativeMethods.rfe_screen_data_get_pixel(_ptr, x, y, &pixel));
            return pixel;
        }
    }

    /// <summary>The screen capture timestamp in UTC.</summary>
    public DateTime TimestampUtc
    {
        get
        {
            unsafe
            {
                long timestamp = 0;
                RfeException.ThrowIfError(NativeMethods.rfe_screen_data_timestamp(_ptr, &timestamp));
                return DateTimeOffset.FromUnixTimeSeconds(timestamp).UtcDateTime;
            }
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
            NativeMethods.rfe_screen_data_free(_ptr);
        }

        GC.SuppressFinalize(this);
    }

    /// <inheritdoc/>
    ~ScreenData()
    {
        Dispose();
    }
}
