#ifndef DHT11_H
#define DHT11_H

#include <DHTesp.h>

void setupDht11(uint8_t);
TempAndHumidity getDht11TempAndHumi();

#endif