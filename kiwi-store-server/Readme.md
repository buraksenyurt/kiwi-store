# Kiwi-Store-Server

Rust ile geliştirilmiş, hafif ve yüksek performanslı bir key-value veri deposudur. Redis benzeri TCP protokolü üzerinden çalışır ve temel komutlar aracılığıyla veri yönetimi sağlar. Şu anki versiyon single node üzerinden işler.

## Özellikler

- Hafif ve hızlı: Rust'ın performans avantajlarından yararlanır.
- Temel komut seti desteği: SET, GET, REMOVE, LIST, STATS, PING.
- TCP üzerinden iletişim: Basit bir metin tabanlı protokol kullanır.
- Kolay entegrasyon: .NET istemcileri için, [NuGet paketi](https://www.nuget.org/packages/KiwiStore/1.0.1) de mevcuttur.

## Kurulum

### Gereksinimler

- Rust (1.60 veya üzeri bir sürüm)
- Cargo (Paket yönetimi için)

### Derleme ve Çalıştırma

```bash
# Reponun Klonlanması

git clone https://github.com/buraksenyurt/kiwi-store.git
cd kiwi-store/kiwi-store-server

# Projenin Derlenmesi
cargo build --release

# Sunucunun Başlatılması
cargo run --release
```

Sunucu varsayılan olarak 127.0.0.1:5555 adresinde dinleme yapar ancak Docker desteği ile farklı port'lardan kullandırılabilir.

### Docker Desteği ile Çalıştırma

```bash
# Docker build
docker build -t kivi-store-server .
```

## Kullanım

Sunucu uygulama aşağıdaki komut setini destekler;

- SET key value: Bir anahtar-değer çifti ekler veya günceller.
- GET key: Belirtilen anahtarın değerini getirir.
- REMOVE key: Belirtilen anahtarı siler.
- LIST: Tüm anahtarları listeler.
- STATS: Depodaki anahtar sayısı ve toplam boyut bilgilerini verir.
- PING: Sunucunun çalışıp çalışmadığını kontrol eder.

```bash
SET SimulationMode On
GET SimulationMode
LIST
REMOVE SimulationMode
STATS
```

## DockerHub Deployment

Kiwi-Store-Server'ın daha kolay kullanılabilmesi için [Docker Hub](https://hub.docker.com/r/burakselim/kiwi-store-server) üzerinden de yayınlamıştır. Docker Hub Deployment işlemleri aşağıdaki adımlar takip edilerek yapılabilir.

```bash
# Önce login olunur
docker login

# Dockerfile' ın olduğu konumda imaj oluşturulur
cd kiwi-store-server
docker build -t [dockerhub-username]/kiwi-store-server:latest .

# Oluşturulan imaj için tag'de hazırlanabilir
docker tag  [dockerhub-username]/kiwi-store-server:latest [dockerhub-username]/kiwi-store-server:v1.0.0

# Oluşan imajlar docker hub üzerine publish edilir
docker push [dockerhub-username]/kiwi-store-server:latest
docker push [dockerhub-username]/kiwi-store-server:v1.0.0

# Eğer private ise Docker Hub repository public hale getirilir
# Hub sayfasında markdown formatında `Overview` kısmı eklenerek kullanımı hakkında bilgi verilir
```
