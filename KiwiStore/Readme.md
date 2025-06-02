# Kiwi-Store .Net Library

Bu paket Kiwi-Store isimli deneysel key-value store uygulamasının .Net kütüphanesidir. Kiwi-Store kullanımı için Dependency Injection desteği sağlanmaktadır. Kullanımı için aşağıdaki adımları takip edebilirsiniz.

## App Settings Desteği

Kiwi-Store uygulaması için appsettings.json dosyasında aşağıdaki gibi bir yapılandırma yapabilirsiniz:

```json
{
  "KiwiStore": {
	"Host": "127.0.0.1",
	"Port":5555
  }
}
```

## Dependency Injection Desteği

Kiwi-Store uygulamasını DI tarafında aşaıdaki farklı şekillerde kullanabilirsiniz:
```csharp
builder.Services.AddKiwiStore(endpoint =>
{
    endpoint = new KiwiEndpoint("localhost", 5555);
});
```

veya

```csharp
builder.Services.AddKiwiStore(builder.Configuration);
```

## Kullanım

Kiwi-Store paketini herhangibir bileşene enjekte ettikten sonra aşağıdaki gibi kullanabilirsiniz:
```csharp
[ApiController]
[Route("[controller]")]
public class ProductController : ControllerBase
{
    private readonly IKiwiStoreClient _store;

    public ProductController(IKiwiStoreClient store)
    {
        _store = store;
    }

    [HttpGet("{key}")]
    public async Task<string> Get(string key)
    {
        return await _store.GetAsync(key);
    }

    [HttpPost("{key}")]
    public async Task<IActionResult> Set(string key, [FromBody] string value)
    {
        await _store.SetAsync(key, value);
        return Ok();
    }
}
```