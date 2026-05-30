using Rfe.Net;

using var rfe = SpectrumAnalyzer.Connect();
if (rfe is null)
{
    Console.Error.WriteLine("Failed to connect to an RF Explorer");
    return 1;
}

try
{
    var sweep = rfe.WaitForNextSweep();
    PrintSweep(sweep, rfe.StartFrequencyHz, rfe.StopFrequencyHz);
    return 0;
}
catch (RfeException)
{
    Console.Error.WriteLine("Failed to wait for next RF Explorer sweep");
    return 1;
}

static void PrintSweep(IReadOnlyList<float> sweep, ulong startHz, ulong stopHz)
{
    Console.WriteLine($"{startHz}-{stopHz} Hz");
    Console.Write("[");

    for (var i = 0; i < sweep.Count; i++)
    {
        if (i != 0)
        {
            Console.Write(", ");
        }

        Console.Write($"{sweep[i]:F1}");
    }

    Console.Write("]");
}
