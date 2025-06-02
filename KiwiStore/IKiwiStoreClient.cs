namespace KiwiStore;

public interface IKiwiStoreClient
{
    Task<string> GetAsync(string key);
    Task<string> SetAsync(string key, string value);
    Task<string> RemoveAsync(string key);
    Task<string> PingAsync();
    Task<string> ListAsync();
    Task<string> StatsAsync();
}
