using KiwiStore;

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddKiwiStoreClient(builder.Configuration);

var app = builder.Build();

app.MapPost("/set", async (IKiwiStoreClient store, KeyValueDto dto) =>
{
    await store.SetAsync(dto.Key, dto.Value);
    return Results.Ok(new { Message = "Key set successfully." });
});

app.MapGet("/get/{key}", async (IKiwiStoreClient store, string key) =>
{
    var value = await store.GetAsync(key);
    return string.IsNullOrEmpty(value)
        ? Results.NotFound(new { Message = "Key not found." })
        : Results.Ok(new { Key = key, Value = value });
});

app.MapDelete("/remove/{key}", async (IKiwiStoreClient store, string key) =>
{
    var response = await store.RemoveAsync(key);
    return Results.Ok(new { Message = response });
});

app.MapGet("/list", async (IKiwiStoreClient store) =>
{
    var response = await store.ListAsync();
    var keys = response.Split('\n', StringSplitOptions.RemoveEmptyEntries);
    return Results.Ok(keys);
});

app.MapGet("/stats", async (IKiwiStoreClient store) =>
{
    var response = await store.StatsAsync();
    return Results.Ok(new { Stats = response });
});

app.MapGet("/ping", async (IKiwiStoreClient store) =>
{
    var result = await store.PingAsync();
    return Results.Ok(new { Result = result });
});


app.Run();

public record KeyValueDto(string Key, string Value);
