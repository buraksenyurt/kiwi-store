# Kiwi-Store

Deneysel ve hafifsiklet bir key-value store çalışmasıdır. İlk sürüm özellikleri;

- Sadece string türden key ve value çiftleri tutulur
- Key bilgisi 8, Value bilgisi 100 karakterden fazla olamaz
- Key-Value bilgileri in-memory saklanır
- Tcp bazlı sunucu ve konsol tabanlı istemci uygulama vardır
- Server side tamamen asenkrondur
- İstemci tarafı için bir CLI uygulaması mevcuttur

## Runtime

Sunucu tarafı için

```bash
docker-compose up -d
```

İstemci tarafı için
