namespace KiwiStore;

using System.Text;
using System.Net.Sockets;
using System.Threading.Tasks;

public class KiwiStoreClient
{
    private readonly string kiwiStoreEndPoint;
    public KiwiStoreClient(string endPoint = "127.0.0.1:5555")
    {
        kiwiStoreEndPoint = endPoint;
    }

    public Task<string> GetAsync(string key)=>SendRequestAsync($"GET {key}");
    public Task<string> SetAsync(string key, string value) => SendRequestAsync($"SET {key} {value}");
    public Task<string> RemoveAsync(string key) => SendRequestAsync($"REMOVE {key}");
    public Task<string> PingAsync() => SendRequestAsync("PING");
    public Task<string> ListAsync() => SendRequestAsync("LIST");
    public Task<string> StatsAsync() => SendRequestAsync("STATS");

    private async Task<string> SendRequestAsync(string message)
    {
        using var client = new TcpClient();
        await client.ConnectAsync(kiwiStoreEndPoint.Split(':')[0], int.Parse(kiwiStoreEndPoint.Split(':')[1]));
        await using var stream = client.GetStream();

        var data = Encoding.UTF8.GetBytes(message + "\n");
        await stream.WriteAsync(data, 0, data.Length);
        var buffer = new byte[1024];
        var bytesRead = await stream.ReadAsync(buffer, 0, buffer.Length);
        return Encoding.UTF8.GetString(buffer, 0, bytesRead).Trim();
    }
}
