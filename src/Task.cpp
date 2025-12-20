#include <Task.h>

TaskHandle_t taskHandle = NULL;
Ticker ticker;
bool taskEnabled = false;

bool initTask()
{
  Serial.println("Init task");
  xTaskCreatePinnedToCore(
      task,
      "task",
      8192,
      NULL,
      5,
      &taskHandle,
      1);
  if (taskHandle == NULL)
  {
    Serial.println("Failed to start task");
    return false;
  }
  ticker.attach(5, triggerTask);
  return true;
}

void triggerTask()
{
  if (taskHandle != NULL)
  {
    xTaskResumeFromISR(taskHandle);
  }
}

void task(void *pvParameters)
{
  Serial.println("Task loop started");
  while (1)
  {
    if (taskEnabled)
    {
      doTask();
    }
    vTaskSuspend(NULL);
  }
}