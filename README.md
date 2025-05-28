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

Load ve Fuzz test yapan kiwi-store-loadtest uygulaması da komut satırından aşağıdaki gibi çalıştırılabilir.

```bash
# 10 istemci, istemci başına 50 komut
cargo run -- -k load -c 10 -s 50

# 100 istemci, istemci başına 1000 komut
cargo run -- -k load -c 100 -s 1000

# 10 istemci, istemci başına 100 geçersiz komut
cargo run -- -k fuzz -c 10 -s 100
```

## Problemler

- Key store'da hiçbir eleman kalmadığında client taraftaki list komutu sonsuz döngüde kalıyor
- Windows ortamında docker container portu zaman zaman kullanımda kalıyor ve yenisi açılamıyor
- Key, Value karakter sınıfı ihlallerinde istemci tarafında mesaj dönse de failed olarak yorumlanamıyor _(Load Test uygulaması için geçerli)_

## Planlanan Ekler

- [x] Load ve Fuzz test yapan bir uygulama
- [ ] Mesajların şifrelenerek iletiminin sağlanması.
- [ ] Monitoring Dashboard
- [ ] .Net ile entegrasyon kütüphanesi
- [ ] Multi-Node Çalışma
