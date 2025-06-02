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
        var configSection = configuration.GetSection("KiwiStore");
        var section = new KiwiStoreSection();
        configSection.Bind(section);
        var endpoint = new KiwiStoreEndpoint(section.Host, section.Port);
        services.AddSingleton(endpoint);
        services.AddSingleton<IKiwiStoreClient, KiwiStoreClient>();
        return services;
    }

}
