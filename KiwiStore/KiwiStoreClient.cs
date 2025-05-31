namespace KiwiStore;

using System.Text;
using System.Net.Sockets;
using System.Threading.Tasks;

public class KiwiStoreClient
{
    private readonly KiwiStoreEndpoint _kiwiStoreEndPoint;
    public KiwiStoreClient(KiwiStoreEndpoint? endPoint)
    {
        _kiwiStoreEndPoint = endPoint ?? KiwiStoreEndpoint.Default;
    }

    public Task<string> GetAsync(string key) => SendRequestAsync($"GET {key}");
    public Task<string> SetAsync(string key, string value) => SendRequestAsync($"SET {key} {value}");
    public Task<string> RemoveAsync(string key) => SendRequestAsync($"REMOVE {key}");
    public Task<string> PingAsync() => SendRequestAsync("PING");
    public Task<string> ListAsync() => SendRequestAsync("LIST");
    public Task<string> StatsAsync() => SendRequestAsync("STATS");

    private async Task<string> SendRequestAsync(string message)
    {
        using var client = new TcpClient();

        await client.ConnectAsync(_kiwiStoreEndPoint.Host, _kiwiStoreEndPoint.Port);
        if (!client.Connected)
            throw new InvalidOperationException("Failed to connect to the KiwiStore server.");

        await using var stream = client.GetStream();

        var data = Encoding.UTF8.GetBytes(message + "\n");
        await stream.WriteAsync(data);
        var buffer = new byte[1024];
        var bytesRead = await stream.ReadAsync(buffer);

        return Encoding.UTF8.GetString(buffer, 0, bytesRead).Trim();
    }
}
