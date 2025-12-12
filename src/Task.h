#ifndef TASK_H
#define TASK_H

#include <Ticker.h>
#include <Arduino.h>

extern void doTask();

extern TaskHandle_t taskHandle;
extern Ticker ticker;
extern bool taskEnabled;

void task(void *);
bool initTask();
void triggerTask();

#endif