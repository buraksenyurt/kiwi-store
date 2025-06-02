using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;

namespace KiwiStore;

public static class ServiceExtensions
{
    public static IServiceCollection AddKiwiStoreClient(this IServiceCollection services, Action<KiwiStoreEndpoint>? configure = null)
    {
        var endpoint = KiwiStoreEndpoint.Default;
        configure?.Invoke(endpoint);
        services.AddSingleton(endpoint);
        services.AddSingleton<IKiwiStoreClient, KiwiStoreClient>();
        return services;
    }

    public static IServiceCollection AddKiwiStoreClient(this IServiceCollection services, IConfiguration configuration)
    {
        if (configuration.GetSection("KiwiStore") is not KiwiStoreSection section)
        {
            throw new InvalidOperationException("KiwiStore configuration section is missing.");
        }

        var host = section.Host ?? "127.0.0.1";
        var port = section.Port;
        var endpoint = new KiwiStoreEndpoint(host, port);

        services.AddSingleton(endpoint);
        services.AddSingleton<IKiwiStoreClient, KiwiStoreClient>();
        return services;
    }
}
