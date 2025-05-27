# Kiwi-Store

Deneysel ve hafifsiklet bir key-value store çalışmasıdır. İlk sürüm özellikleri;

- Sadece string türden key ve value çiftleri tutulur
- Key bilgisi 8, Value bilgisi 100 karakterden fazla olamaz
- Key-Value bilgileri in-memory saklanır
- Tcp bazlı sunucu ve konsol tabanlı istemci uygulama vardır
- Server side tamamen asenkrondur
- İstemci tarafı için bir CLI uygulaması mevcuttur
- Server side için load, fuzz test işleten bir test koşucu uygulaması vardır

## Runtime

Sunucu tarafı için

```bash
docker-compose up -d
```

İstemci tarafı için komut satır aracı kullanılabilir.

```bash
# cargo run ile
# kiwi-store-client klasöründe
cargo run -- set smtp fake
cargo run -- get smtp
cargo run -- set conn "data source=localhost;database=Northwind;integrated security=sspi"
cargo run -- list
cargo run -- remove smtp
```

## Problemler

- Key store'da hiçbir eleman kalmadığında client taraftaki list komutu sonsuz döngüde kalıyor

## Planlanan Ekler

- [x] Load ve Fuzz test yapan bir uygulama
- [ ] Monitoring Dashboard
- [ ] .Net ile entegrasyon kütüphanesi
- [ ] Multi-Node Çalışma
