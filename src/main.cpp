#include <Arduino.h>
#include <WiFi.h>
#include <MQ135.h>
#include <DHT11.h>
#include <Task.h>
#include <time.h>
#include <PubSubClient.h>
#include <ArduinoJson.h>

#define DHTPIN 4
#define DHTTYPE DHT11

#define PIN_MQ135 34

MQ135 mq135_sensor(PIN_MQ135);

String ssid = "iot_network";
String passphrase = "@Iotnet914";

WiFiClient espClient;
PubSubClient client(espClient);

const char *mqttServer = "192.168.137.1";
uint16_t const mqttPort = 1883;
const char *mqttTopic = "iot/hat";

const char *ntpServer = "time.google.com";
long const gmtOffset_sec = 0;
int const daylightOffset_sec = 0;

void doTask();
void setupWiFi();
void readAirQuality(float temp, float humi, float &rZero, float &correctedRZero, float &resistance, float &ppm, float &correctedPPM);
void printAirQuality(float rZero, float correctedRZero, float resistance, float ppm, float correctedPPM);
unsigned long getEpochTime();
void reconnectMQTT();
void callback(char* topic, byte* payload, unsigned int length);

void setup()
{
  Serial.begin(115200);

  analogReadResolution(10);
  analogSetAttenuation(ADC_11db);
  setupWiFi();
  setupDht11(DHTPIN);
  initTask();
  taskEnabled = true;
  client.setServer(mqttServer, mqttPort);
  client.setCallback(callback);
  configTime(gmtOffset_sec, daylightOffset_sec, ntpServer);
}

void loop()
{
  yield();
}

void doTask()
{
  if (WiFi.status() == WL_CONNECTED)
  {
    if (!client.connected()) {
      reconnectMQTT();
    }
    client.loop();

    TempAndHumidity tah = getDht11TempAndHumi();
    Serial.println("Temp: " + String(tah.temperature) + " °C, "
    "Humi: " +
    String(tah.humidity));
    float rZero;
    float correctedRZero;
    float resistance;
    float ppm;
    float correctedPPM;
    readAirQuality(tah.temperature, tah.humidity, rZero, correctedRZero, resistance, ppm, correctedPPM);
    unsigned long now = getEpochTime();
    if (now < 100000) {
      Serial.println("NTP is not available, skip...");
      // return;
    }
    JsonDocument doc;
    doc["timestamp"] = now;
    doc["temperature"] = tah.temperature;
    doc["humidity"] = tah.humidity;
    doc["r_zero"] = rZero;
    doc["corrected_r_zero"] = correctedRZero;
    doc["resistance"] = resistance;
    doc["ppm"] = ppm;
    doc["corrected_ppm"] = correctedPPM;

    String buf;
    serializeJson(doc, buf);
    Serial.println(buf);
    if (client.publish(mqttTopic, buf.c_str())) {
      Serial.print(">> Gui MQTT: ");
      Serial.println(buf);
    } else {
      Serial.println("!! Gui MQTT THAT BAI");
    }
    client.subscribe(mqttTopic);
  }
  else
  {
    Serial.println("WiFi Disconnected");
  }
}

String translateEncryptionType(wifi_auth_mode_t encryptionType)
{
  switch (encryptionType)
  {
  case (WIFI_AUTH_OPEN):
    return "Open";
  case (WIFI_AUTH_WEP):
    return "WEP";
  case (WIFI_AUTH_WPA_PSK):
    return "WPA_PSK";
  case (WIFI_AUTH_WPA2_PSK):
    return "WPA2_PSK";
  case (WIFI_AUTH_WPA_WPA2_PSK):
    return "WPA_WPA2_PSK";
  case (WIFI_AUTH_WPA2_ENTERPRISE):
    return "WPA2_ENTERPRISE";
  default:
    return "Unknown";
  }
}

void setupWiFi()
{
  WiFi.mode(WIFI_STA);
  WiFi.disconnect();
  delay(100);
  int n = WiFi.scanNetworks();
  Serial.println("Scan done");
  if (n == 0)
  {
    Serial.println("no networks found");
  }
  else
  {
    Serial.print(n);
    Serial.println(" networks found");
    Serial.println("------------------------------------------------");
    Serial.printf("%-32s | %-10s | %-5s\n", "SSID", "AUTH", "RSSI");
    Serial.println("------------------------------------------------");

    for (int i = 0; i < n; ++i)
    {
      Serial.printf("%-32s | %-10s | %d dBm\n",
                    WiFi.SSID(i).c_str(),
                    translateEncryptionType(WiFi.encryptionType(i)).c_str(),
                    WiFi.RSSI(i));
      delay(10);
    }
  }
  Serial.println("");
  WiFi.scanDelete();

  IPAddress dns(8, 8, 8, 8);
  WiFi.config(INADDR_NONE, INADDR_NONE, INADDR_NONE, dns);

  WiFi.begin(ssid, passphrase, 6);
  while (WiFi.status() != WL_CONNECTED)
  {
    delay(100);
    Serial.print(".");
  }
  Serial.println(" Connected!");
  Serial.print("IP Address: ");
  Serial.println(WiFi.localIP());
  Serial.print("DNS IP: ");
  Serial.println(WiFi.dnsIP());
}

void readAirQuality(float temp, float humi, float &rZero, float &correctedRZero, float &resistance, float &ppm, float &correctedPPM)
{
  Serial.printf("Temp = %f, Humi = %f\n", temp, humi);
  rZero = mq135_sensor.getRZero();
  correctedRZero = mq135_sensor.getCorrectedRZero(temp, humi);
  resistance = mq135_sensor.getResistance();
  ppm = mq135_sensor.getPPM();
  correctedPPM = mq135_sensor.getCorrectedPPM(temp, humi);
  printAirQuality(rZero, correctedRZero, resistance, ppm, correctedPPM);
}

void printAirQuality(float rZero, float correctedRZero, float resistance, float ppm, float correctedPPM)
{
  Serial.print("MQ135 RZero: ");
  Serial.print(rZero);
  Serial.print("\t Corrected RZero: ");
  Serial.print(correctedRZero);
  Serial.print("\t Resistance: ");
  Serial.print(resistance);
  Serial.print("\t PPM: ");
  Serial.print(ppm);
  Serial.print("ppm");
  Serial.print("\t Corrected PPM: ");
  Serial.print(correctedPPM);
  Serial.println("ppm");
}

unsigned long getEpochTime() {
  time_t now;
  time(&now);
  return now;
}

void reconnectMQTT() {
  if (!client.connected()) {
    Serial.print("Ket noi MQTT...");
    String clientId = "ESP32-" + String(random(0xffff), HEX);
    if (client.connect(clientId.c_str())) {
      Serial.println("OK");
    } else {
      Serial.print("Loi rc=");
      Serial.println(client.state());
    }
  }
}


void callback(char* topic, byte* payload, unsigned int length) {
  Serial.print("<< Message arrived [");
  Serial.print(topic);
  Serial.print("] ");
  for (int i=0;i<length;i++) {
    Serial.print((char)payload[i]);
  }
  Serial.println();
}
