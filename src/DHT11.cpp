#include <dht11.h>

DHTesp dht11;

void setupDht11(uint8_t dht11Pin) {
  dht11.setup(dht11Pin, DHTesp::DHT11);
}

TempAndHumidity getDht11TempAndHumi() {
  TempAndHumidity cur = dht11.getTempAndHumidity();
  return cur;
}