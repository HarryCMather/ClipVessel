using System;

namespace ClipVesselClient.Models.DTO;

/// <summary>
/// Model to represent a log, which will be transferred from the Rust backend to the Blazor frontend.
/// </summary>
public sealed class Log
{
    /// <summary>
    /// The DateTime when the event occurred.
    /// </summary>
    public DateTimeOffset TimeStamp { get; set; }

    /// <summary>
    /// The type of event, this is designed to differentiate between application logs/faults and Handbrake logs.
    /// </summary>
    public string Type { get; set; }

    /// <summary>
    /// The message to log for the event.
    /// </summary>
    public string Message { get; set; }
}
