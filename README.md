# Rust Log İzleme Aracı

Bu proje, Rust ile geliştirilmiş, uzak sunuculardaki logları gerçek zamanlı olarak izlemeyi sağlayan bir araçtır. Amacımız, geliştiricilere ve sistem yöneticilerine uygulamalarının loglarını daha verimli bir şekilde takip etmelerine yardımcı olmaktır.

## Özellikler

*   **Güvenli Erişim:** Loglar, private ve public key mekanizması ile güvence altına alınmıştır. Sadece yetkili kullanıcılar loglara erişebilir.
*   **Gerçek Zamanlı İzleme:** Loglar anlık olarak izlenebilir.
*   **Log Seviyesi Değiştirme:** Kullanıcılar, istedikleri log seviyesini (örn. DEBUG, INFO, ERROR) seçebilirler.
*   **Log Askıya Alma (Suspend):** Kullanıcılar, belirli sınıf veya servislerden gelen logları askıya alabilirler.
*   **Log Koşulu (Condition):** Kullanıcılar, belirli sınıf veya servislerden gelen loglara etiket ekleyebilirler.
*   **Log Önbelleği (Cache):** Askıya alma ve koşul özellikleri için önbellekleme mekanizması kullanılmıştır.

## Amaçlar

*   **Güvenlik:** Loglara yetkisiz erişimi engellemek.
*   **Performans:** Logları hızlı ve verimli bir şekilde izlemek.
*   **Esneklik:** Kullanıcılara log izleme üzerinde kontrol imkanı sunmak.
*   **Kullanılabilirlik:** Aracın kolayca kurulabilir ve kullanılabilir olmasını sağlamak.

## Kurulum

```bash
cargo install rust-log-izleme-araci
Kullanım
Anahtar Çifti Oluşturma:

Bash

rust-log-izleme-araci anahtar-olustur
Bu komut, bir public ve private key çifti oluşturur. Private key'i güvenli bir yerde saklayın.

Log İzleme:

Bash

rust-log-izleme-araci izle --public-key <public_key> --websocket-url <websocket_url>
Bu komut, belirtilen WebSocket URL'sine bağlanarak logları izlemeye başlar.

Yapılandırma
Yapılandırma dosyası (config.yaml) kullanılarak aracın davranışı özelleştirilebilir.

YAML

websocket_url: "wss://[example.com/logs](https://www.google.com/search?q=https://example.com/logs)"
log_level: "INFO"
suspend:
  - "com.example.Servis1"
condition:
  - class: "com.example.Class1"
    tag: "Önemli"
Geliştirme
Katkılarınızdan memnuniyet duyarız! Lütfen pull requestlerinizi gönderin.

Lisans
MIT Lisansı

İletişim
Sorularınız veya geri bildirimleriniz için lütfen bizimle iletişime geçin.