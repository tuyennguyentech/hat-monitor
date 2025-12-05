#include <Arduino.h>
#include <WiFi.h>

#define LED_PIN 2

const char* ssid = "Tuyen Bao Tung";
const char* password = "@tuyenbaotung123";

void setup() {
  pinMode(LED_PIN, OUTPUT);
  Serial.begin(115200);
  Serial.println("Hello ESP32 CH340!");

  WiFi.mode(WIFI_STA);
  WiFi.disconnect();
  delay(100);

  // --- PHẦN 1: LIỆT KÊ (SCAN) WIFI ---
  Serial.println("\n--- BAT DAU SCAN WIFI ---");
  int n = WiFi.scanNetworks();

  if (n == 0) {
    Serial.println("Khong tim thay mang WiFi nao!");
  } else {
    Serial.print("Tim thay ");
    Serial.print(n);
    Serial.println(" mang:");

    for (int i = 0; i < n; ++i) {
      // In ra: [Số thứ tự] Tên mạng (Cường độ tín hiệu)
      Serial.print(i + 1);
      Serial.print(": ");
      Serial.print(WiFi.SSID(i));
      Serial.print(" (");
      Serial.print(WiFi.RSSI(i));
      Serial.println(" dBm)");
      delay(10);
    }
  }
  Serial.println("-------------------------");

  // Xóa bộ nhớ scan để giải phóng RAM
  WiFi.scanDelete();


  // --- PHẦN 2: KẾT NỐI (CONNECT) ---
  Serial.print("Dang ket noi vao WiFi: ");
  Serial.println(ssid);

  WiFi.begin(ssid, password);

  // Vòng lặp chờ kết nối
  // WiFi.status() trả về WL_CONNECTED khi thành công
  int attempts = 0;
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");

    // Nếu chờ quá lâu (ví dụ 20 lần = 10 giây) mà không được thì báo lỗi
    attempts++;
    if (attempts > 20) {
      Serial.println("\nKet noi that bai! Kiem tra lai Mat khau hoac Ten WiFi.");
      // Dừng luôn ở đây hoặc reset
      return;
    }
  }

  Serial.println("");
  Serial.println("Da ket noi thanh cong!");
  Serial.print("Dia chi IP cua ESP32: ");
  Serial.println(WiFi.localIP());
}

void loop() {
  digitalWrite(LED_PIN, HIGH);
  Serial.println("LED ON");
  delay(1000);

  digitalWrite(LED_PIN, LOW);
  Serial.println("LED OFF");
  delay(1000);

  if(WiFi.status() == WL_CONNECTED) {
      // Mạng ổn định, làm việc gì đó...
  } else {
      Serial.println("Mat ket noi WiFi!");
      WiFi.reconnect(); // Thử kết nối lại
      delay(5000);
  }

  delay(1000);
}
