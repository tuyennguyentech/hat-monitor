package com.example.mqttmonitor

import android.graphics.Color
import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import com.example.mqttmonitor.databinding.ActivityMainBinding
import com.github.mikephil.charting.charts.LineChart
import com.github.mikephil.charting.components.XAxis
import com.github.mikephil.charting.data.Entry
import com.github.mikephil.charting.data.LineData
import com.github.mikephil.charting.data.LineDataSet
import org.eclipse.paho.client.mqttv3.*
import org.eclipse.paho.client.mqttv3.persist.MemoryPersistence
import org.json.JSONObject

class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding
    private lateinit var mqttClient: MqttClient

    // Cấu hình - Thay IP máy tính của bạn vào đây
    private val serverUri = "tcp://192.168.137.1:1883"
    private val topic = "iot/hat"
    private val maxEntries = 100 // Giới hạn 100 điểm trên đồ thị

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        initDashboard()
        setupCharts()
        setupMqtt()
    }

    private fun initDashboard() {
        binding.cardTemp.tvLabel.text = "Nhiệt độ"
        binding.cardTemp.tvUnit.text = "°C"

        binding.cardHumi.tvLabel.text = "Độ ẩm"
        binding.cardHumi.tvUnit.text = "%"

        binding.cardPpm.tvLabel.text = "Khí Gas (PPM)"
        binding.cardPpm.tvUnit.text = "ppm"

        binding.cardCorrected.tvLabel.text = "PPM Hiệu chỉnh"
        binding.cardCorrected.tvUnit.text = "ppm"
    }

    private fun setupCharts() {
        configureChart(binding.chartTemp, "Nhiệt độ", Color.RED)
        configureChart(binding.chartHumi, "Độ ẩm", Color.BLUE)
        configureChart(binding.chartPpm, "Khí Gas", Color.parseColor("#2E7D32"))
    }

    private fun configureChart(chart: LineChart, label: String, color: Int) {
        val entries = mutableListOf<Entry>()
        val dataSet = LineDataSet(entries, label).apply {
            this.color = color
            setDrawCircles(false)
            setDrawValues(false)
            lineWidth = 2f
            mode = LineDataSet.Mode.CUBIC_BEZIER
        }

        chart.apply {
            description.isEnabled = false
            setTouchEnabled(true)
            xAxis.position = XAxis.XAxisPosition.BOTTOM
            xAxis.setDrawGridLines(false)
            axisRight.isEnabled = false
            data = LineData(dataSet)
            invalidate()
        }
    }

    private fun setupMqtt() {
        val clientId = "AndroidClient_${System.currentTimeMillis()}"
        try {
            mqttClient = MqttClient(serverUri, clientId, MemoryPersistence())
            mqttClient.setCallback(object : MqttCallback {
                override fun messageArrived(topic: String?, message: MqttMessage?) {
                    val payload = String(message!!.payload)
                    parseAndNotify(payload)
                }
                override fun connectionLost(cause: Throwable?) {
                    runOnUiThread { binding.tvStatus.text = "Mất kết nối!" }
                }
                override fun deliveryComplete(token: IMqttDeliveryToken?) {}
            })

            Thread {
                try {
                    val options = MqttConnectOptions().apply { isCleanSession = true }
                    mqttClient.connect(options)
                    mqttClient.subscribe(topic, 1)
                    runOnUiThread {
                        binding.tvStatus.text = "Trực tuyến: $topic"
                        Toast.makeText(this@MainActivity, "Connected", Toast.LENGTH_SHORT).show()
                    }
                } catch (e: Exception) { e.printStackTrace() }
            }.start()
        } catch (e: Exception) { e.printStackTrace() }
    }

    private fun parseAndNotify(jsonData: String) {
        try {
            val json = JSONObject(jsonData)
            val temp = json.optDouble("temperature", 0.0).toFloat()
            val humi = json.optDouble("humidity", 0.0).toFloat()
            val ppm = json.optDouble("corrected_ppm", 0.0).toFloat()

            runOnUiThread {
                // Cập nhật số
                binding.cardTemp.tvValue.text = String.format("%.1f", temp)
                binding.cardHumi.tvValue.text = String.format("%.1f", humi)
                binding.cardPpm.tvValue.text = String.format("%.2f", json.optDouble("ppm"))
                binding.cardCorrected.tvValue.text = String.format("%.2f", ppm)

                // Cập nhật đồ thị
                addEntry(binding.chartTemp, temp)
                addEntry(binding.chartHumi, humi)
                addEntry(binding.chartPpm, ppm)
            }
        } catch (e: Exception) { e.printStackTrace() }
    }

    private fun addEntry(chart: LineChart, value: Float) {
        val data = chart.data
        data?.let {
            val set = it.getDataSetByIndex(0)
            it.addEntry(Entry(set.entryCount.toFloat(), value), 0)

            if (set.entryCount > maxEntries) {
                set.removeEntry(0)
                for (i in 0 until set.entryCount) {
                    set.getEntryForIndex(i).x = i.toFloat()
                }
            }

            it.notifyDataChanged()
            chart.notifyDataSetChanged()
            chart.setVisibleXRangeMaximum(maxEntries.toFloat())
            chart.moveViewToX(it.entryCount.toFloat())
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        if (::mqttClient.isInitialized && mqttClient.isConnected) mqttClient.disconnect()
    }
}