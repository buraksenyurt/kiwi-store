namespace KiwiStore;

public class KiwiStoreEndpoint
{
    public string Host { get; set; }
    public int Port { get; set; }
    public KiwiStoreEndpoint(string host, int port)
    {
        Host = host ?? throw new ArgumentNullException(nameof(host));
        if (port <= 5000 || port > 65535)
            throw new ArgumentOutOfRangeException(nameof(port), "Port must be between 5000 and 65535.");

        Port = port;
    }
    public override string ToString()
    {
        return $"{Host}:{Port}";
    }
    public static KiwiStoreEndpoint Default => new("127.0.0.1", 5555);
}
