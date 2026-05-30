using System.Runtime.InteropServices;
using System.Text;
using CsBindgen;

namespace Rfe.Net;

internal static unsafe class NativeHelpers
{
    internal delegate Result FillBuffer(byte* buffer, nuint length);

    internal static byte[] ToNullTerminatedUtf8(string value)
    {
        var bytes = Encoding.UTF8.GetBytes(value);
        Array.Resize(ref bytes, bytes.Length + 1);
        return bytes;
    }

    internal static string ReadString(Func<nuint> getLength, FillBuffer fill)
    {
        var length = getLength();
        if (length == 0)
        {
            return "";
        }

        var buffer = new byte[length];
        fixed (byte* ptr = buffer)
        {
            RfeException.ThrowIfError(fill(ptr, length));
        }

        return Encoding.UTF8.GetString(buffer, 0, buffer.Length - 1);
    }

    internal static string? ReadOptionalString(Func<nuint> getLength, FillBuffer fill)
    {
        var length = getLength();
        if (length == 0)
        {
            return null;
        }

        return ReadString(() => length, fill);
    }

    internal static IReadOnlyList<string> PortNames()
    {
        nuint count = 0;
        var names = NativeMethods.rfe_port_names(&count);
        if (names == null)
        {
            return [];
        }

        try
        {
            var result = new string[(int)count];
            for (var i = 0; i < result.Length; i++)
            {
                result[i] = Marshal.PtrToStringUTF8((IntPtr)names[i]) ?? "";
            }

            return result;
        }
        finally
        {
            NativeMethods.rfe_free_port_names(names, count);
        }
    }
}
