using CsBindgen;

namespace Rfe.Net;

/// <summary>An error returned by the native rfe library.</summary>
public sealed class RfeException : Exception
{
    internal RfeException(Result result)
        : base(GetMessage(result))
    {
        Result = (RfeResult)result;
    }

    /// <summary>The native result code.</summary>
    public RfeResult Result { get; }

    internal static void ThrowIfError(Result result)
    {
        if (result != CsBindgen.Result.Success)
        {
            throw new RfeException(result);
        }
    }

    private static string GetMessage(Result result) =>
        result switch
        {
            CsBindgen.Result.IncompatibleFirmwareError => "The connected device reported unsupported or incompatible firmware.",
            CsBindgen.Result.InvalidInputError => "An argument was invalid.",
            CsBindgen.Result.InvalidOperationError => "The requested operation is not valid for the current device state.",
            CsBindgen.Result.IoError => "A serial port or operating system I/O error occurred.",
            CsBindgen.Result.NoData => "The requested data has not been received from the device.",
            CsBindgen.Result.NullPtrError => "A required pointer argument was null.",
            CsBindgen.Result.TimeoutError => "The device did not respond before the operation timed out.",
            _ => "An unknown rfe error occurred.",
        };
}

/// <summary>Result code returned by fallible native functions.</summary>
public enum RfeResult
{
    /// <summary>The function completed successfully.</summary>
    Success = 0,
    /// <summary>The connected device reported unsupported or incompatible firmware.</summary>
    IncompatibleFirmwareError,
    /// <summary>An argument was invalid.</summary>
    InvalidInputError,
    /// <summary>The requested operation is not valid for the current device state.</summary>
    InvalidOperationError,
    /// <summary>A serial port or operating system I/O error occurred.</summary>
    IoError,
    /// <summary>The requested data has not been received from the device.</summary>
    NoData,
    /// <summary>A required pointer argument was null.</summary>
    NullPtrError,
    /// <summary>The device did not respond before the operation timed out.</summary>
    TimeoutError,
}
