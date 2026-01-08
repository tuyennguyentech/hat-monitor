# 🌡️ HAT Monitor - IoT Air Quality Monitoring System

Hệ thống giám sát chất lượng không khí sử dụng ESP32, cảm biến DHT11 và MQ135, truyền dữ liệu qua MQTT để hiển thị trên web dashboard và ứng dụng Android.

## 📋 Tổng Quan

**HAT Monitor** là một giải pháp IoT hoàn chỉnh để giám sát:
- 🌡️ **Nhiệt độ** (Temperature)
- 💧 **Độ ẩm** (Humidity)  
- 🌫️ **Chất lượng không khí** (Air Quality - PPM CO2, NH3, Benzene, Khói...)

### Kiến Trúc Hệ Thống

```
┌─────────────────────────────────────────────────────────────┐
│                       ESP32 Firmware                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   DHT11     │  │   MQ135     │  │    WiFi     │         │
│  │  (GPIO 4)   │  │  (GPIO 34)  │  │   Module    │         │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘         │
│         │                 │                 │                │
│         └────────┬────────┘                 │                │
│                  ▼                          │                │
│          ┌───────────────┐                 │                │
│          │  FreeRTOS     │                 │                │
│          │  Task (5s)    │                 │                │
│          └───────┬───────┘                 │                │
│                  ▼                          ▼                │
│          ┌──────────────────────────────────┐               │
│          │      MQTT Publisher              │               │
│          │   Topic: iot/hat                 │               │
│          └──────────────┬───────────────────┘               │
└─────────────────────────┼─────────────────────────────────-─┘
                          │
                          ▼
              ┌───────────────────────┐
              │   MQTT Broker         │
              │   192.168.137.1:1883  │
              └───────────┬───────────┘
                          │
           ┌──────────────┼──────────────┐
           ▼              ▼               ▼
   ┌──────────────┐ ┌──────────┐ ┌─────────────┐
   │ Rust Server  │ │   Web    │ │   Android   │
   │   Backend    │ │ Dashboard│ │     App     │
   └──────────────┘ └──────────┘ └─────────────┘
```

## 🔧 Phần Cứng Yêu Cầu

### ESP32 Firmware (`src/`)
| Linh kiện | Số lượng | Kết nối | Mô tả |
|-----------|----------|---------|-------|
| ESP32 DevKit v1 | 1 | - | Vi điều khiển chính |
| DHT11 | 1 | GPIO 4 | Cảm biến nhiệt độ & độ ẩm |
| MQ135 | 1 | GPIO 34 (ADC) | Cảm biến chất lượng không khí |
| Nguồn 5V | 1 | VIN/GND | Cấp nguồn cho ESP32 |

### Sơ Đồ Kết Nối

```
DHT11          ESP32
─────          ─────
VCC    ───→    3.3V
DATA   ───→    GPIO 4
GND    ───→    GND

MQ135          ESP32
─────          ─────
VCC    ───→    5V
A0     ───→    GPIO 34
GND    ───→    GND
```

## 📦 Cài Đặt

### 1. Firmware ESP32

#### Yêu cầu
- [PlatformIO](https://platformio.org/) (IDE hoặc CLI)
- ESP32 DevKit v1
- Cáp USB Type-C/Micro-USB

#### Các bước cài đặt

1. **Clone repository:**
```bash
git clone https://github.com/your-username/hat-monitor.git
cd hat-monitor
```

2. **Cấu hình WiFi & MQTT:**
Mở file `src/main.cpp` và chỉnh sửa:
```cpp
String ssid = "iot_network";          // Tên WiFi của bạn
String passphrase = "@Iotnet914";     // Mật khẩu WiFi

const char *mqttServer = "192.168.137.1";  // Địa chỉ MQTT broker
uint16_t const mqttPort = 1883;            // Port MQTT
const char *mqttTopic = "iot/hat";         // MQTT topic
```

3. **Build & Upload:**
```bash
# Build project
platformio run

# Upload lên ESP32
platformio run --target upload

# Mở Serial Monitor để debug
platformio device monitor
```

#### Thư viện sử dụng (tự động cài đặt qua platformio.ini)
- `phoenix1747/MQ135@^1.1.1` - Thư viện cảm biến MQ135
- `beegee-tokyo/DHT sensor library for ESPx@^1.19` - Thư viện DHT11
- `knolleary/PubSubClient@^2.8` - MQTT client
- `bblanchon/ArduinoJson@^7.4.2` - Xử lý JSON

### 2. Backend Server (Rust)

```bash
cd rust/server
cargo build --release
cargo run
```

### 3. Web Dashboard (Leptos)

```bash
cd rust/frontend
trunk serve
```

Truy cập: `http://localhost:8080`

### 4. Android App

Mở thư mục `MQTTMonitor/` bằng Android Studio và build.

## 📡 Giao Thức MQTT

### Publish Topic: `iot/hat`

**Định dạng JSON:**
```json
{
  "timestamp": 1736294400,
  "temperature": 25.5,
  "humidity": 60.2,
  "r_zero": 150.3,
  "corrected_r_zero": 148.7,
  "resistance": 200.5,
  "ppm": 450.2,
  "corrected_ppm": 445.8
}
```

**Giải thích các trường:**
- `timestamp`: Unix timestamp (giây từ 1/1/1970)
- `temperature`: Nhiệt độ (°C)
- `humidity`: Độ ẩm (%)
- `r_zero`: Điện trở cảm biến trong không khí sạch (Ω)
- `corrected_r_zero`: RZero hiệu chỉnh theo T°/độ ẩm (Ω)
- `resistance`: Điện trở hiện tại của cảm biến (Ω)
- `ppm`: Nồng độ khí (parts per million)
- `corrected_ppm`: PPM hiệu chỉnh (chính xác hơn)

**Tần suất:** Mỗi 5 giây

## 📂 Cấu Trúc Thư Mục

```
hat-monitor/
├── src/                          # Firmware ESP32 (C++)
│   ├── main.cpp                  # File chính, logic chương trình
│   ├── DHT11.h/cpp              # Module cảm biến DHT11
│   └── Task.h/cpp               # Quản lý FreeRTOS task
│
├── rust/                         # Backend & Frontend (Rust)
│   ├── app/                      # Leptos components
│   │   └── src/
│   │       ├── connection_badge.rs  # Hiển thị trạng thái kết nối
│   │       ├── graph.rs             # Biểu đồ dữ liệu
│   │       ├── temperature.rs        # Component nhiệt độ
│   │       ├── humidity.rs          # Component độ ẩm
│   │       └── ppm.rs               # Component chất lượng không khí
│   │
│   ├── server/                   # MQTT subscriber & API server
│   │   └── src/
│   │       ├── main.rs           # Server chính
│   │       └── mqttc_worker.rs   # Worker xử lý MQTT
│   │
│   ├── frontend/                 # Web UI
│   ├── types/                    # Shared types
│   └── end2end/                  # Playwright tests
│
├── MQTTMonitor/                  # Android app (Kotlin)
│   └── app/src/main/java/com/example/mqttmonitor/
│       └── MainActivity.kt
│
├── platformio.ini                # Cấu hình PlatformIO
├── Cargo.toml                    # Workspace Rust
└── README.md                     # File này
```

## 🔍 Chi Tiết Firmware

### Luồng Hoạt Động

1. **Khởi động** (`setup()`):
   ```
   [Khởi tạo Serial] → [Cấu hình ADC] → [Kết nối WiFi] 
   → [Khởi tạo DHT11] → [Tạo FreeRTOS Task] 
   → [Kết nối MQTT] → [Đồng bộ NTP]
   ```

2. **Vòng lặp chính** (mỗi 5 giây):
   ```
   [Timer Trigger] → [Đọc DHT11] → [Đọc MQ135] 
   → [Tạo JSON] → [Publish MQTT] → [Task Sleep]
   ```

### FreeRTOS Task Management

- **Task Priority:** 5 (ưu tiên trung bình)
- **Stack Size:** 8192 bytes
- **Core Pinning:** Core 1 (để Core 0 xử lý WiFi/Bluetooth)
- **Trigger:** Timer Ticker mỗi 5 giây
- **Cơ chế:** Task suspend/resume (tiết kiệm năng lượng)

### Hiệu Chỉnh Cảm Biến

**MQ135** bị ảnh hưởng bởi nhiệt độ và độ ẩm. Firmware tự động hiệu chỉnh:

```cpp
// PPM không hiệu chỉnh (kém chính xác)
float ppm = mq135_sensor.getPPM();

// PPM hiệu chỉnh (khuyến nghị sử dụng)
float correctedPPM = mq135_sensor.getCorrectedPPM(temp, humi);
```

### Debug Serial Output

```
Scan done
2 networks found
------------------------------------------------
SSID                             | AUTH       | RSSI 
------------------------------------------------
iot_network                      | WPA2_PSK   | -45 dBm
guest_wifi                       | Open       | -67 dBm
------------------------------------------------
...... Connected!
IP Address: 192.168.137.100
DNS IP: 8.8.8.8
Init task
Task loop started
Temp: 25.5 °C, Humi: 60.2
Temp = 25.500000, Humi = 60.200000
MQ135 RZero: 150.3	 Corrected RZero: 148.7	 Resistance: 200.5	 PPM: 450.2ppm	 Corrected PPM: 445.8ppm
{"timestamp":1736294400,"temperature":25.5,"humidity":60.2,"r_zero":150.3,"corrected_r_zero":148.7,"resistance":200.5,"ppm":450.2,"corrected_ppm":445.8}
>> Gui MQTT: {...}
```

## 🚀 Sử Dụng

### 1. Giám sát qua Serial Monitor
```bash
platformio device monitor
```

### 2. Xem dữ liệu trên Web Dashboard
```
http://localhost:8080
```

### 3. Subscribe MQTT (test)
```bash
# Cài đặt mosquitto-clients
sudo apt install mosquitto-clients

# Subscribe topic
mosquitto_sub -h 192.168.137.1 -t iot/hat -v
```

## 📊 Ngưỡng Chất Lượng Không Khí

| PPM (CO2) | Mức độ | Mô tả |
|-----------|--------|-------|
| < 400 | Tốt | Không khí sạch |
| 400-1000 | Chấp nhận được | Bình thường |
| 1000-2000 | Kém | Cần thông gió |
| 2000-5000 | Xấu | Gây đau đầu, buồn ngủ |
| > 5000 | Nguy hiểm | Ảnh hưởng sức khỏe nghiêm trọng |

## 🛠️ Troubleshooting

### ESP32 không kết nối được WiFi
- Kiểm tra SSID và password trong `main.cpp`
- Đảm bảo WiFi là 2.4GHz (ESP32 không hỗ trợ 5GHz)
- Kiểm tra cường độ tín hiệu (RSSI > -70 dBm)

### MQTT publish thất bại
- Kiểm tra MQTT broker đang chạy: `mosquitto -v`
- Kiểm tra IP và port trong `main.cpp`
- Kiểm tra firewall không chặn port 1883

### Giá trị cảm biến bất thường
- **DHT11** trả về -999: Kiểm tra kết nối GPIO 4
- **MQ135** trả về 0: Kiểm tra kết nối GPIO 34
- **MQ135 cần warm-up**: Đợi 24-48 giờ sau khi cấp nguồn lần đầu

### NTP không đồng bộ
```
NTP is not available, skip...
```
- Kiểm tra kết nối internet
- Thử đổi NTP server: `pool.ntp.org`, `time.windows.com`

## 🤝 Đóng Góp

Mọi đóng góp đều được chào đón! Vui lòng:
1. Fork repository
2. Tạo branch mới: `git checkout -b feature/amazing-feature`
3. Commit thay đổi: `git commit -m 'Add amazing feature'`
4. Push lên branch: `git push origin feature/amazing-feature`
5. Tạo Pull Request

## 📝 License

Dự án này được phân phối dưới giấy phép MIT. Xem file `LICENSE` để biết thêm chi tiết.

## 👥 Tác Giả

- **Developer** - IoT & Embedded Systems

## 🙏 Lời Cảm Ơn

- [PlatformIO](https://platformio.org/) - Build system
- [Leptos](https://leptos.dev/) - Rust web framework
- [Eclipse Mosquitto](https://mosquitto.org/) - MQTT broker
- Cộng đồng ESP32 Arduino

---

**⭐ Nếu project hữu ích, hãy cho một star nhé!**

