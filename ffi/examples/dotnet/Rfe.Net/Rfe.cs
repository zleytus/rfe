using CsBindgen;

namespace Rfe.Net;

/// <summary>Common RF Explorer helpers.</summary>
public static class RfExplorer
{
    /// <summary>Returns whether the platform RF Explorer USB serial driver appears to be installed.</summary>
    public static bool IsDriverInstalled => NativeMethods.rfe_is_driver_installed();

    /// <summary>Returns the RF Explorer serial port names visible on this system.</summary>
    public static IReadOnlyList<string> PortNames() => NativeHelpers.PortNames();
}
