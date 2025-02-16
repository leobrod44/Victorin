# This file is executed on every boot (including wake-boot from deepsleep)
#import esp
#esp.osdebug(None)
#import webrepl
#webrepl.start()

import network
import socket
import ujson as json
import urequests
import machine
import time

# Wi-Fi Configuration
WIFI_SSID = "Balais"
WIFI_PASSWORD = "dUst102!"

# Humidity Sensor (ADC)
ADC_PIN_1 = 33
ADC_PIN_2 = 32

# Valve GPIO
VALVE_PIN = 2

# Server Configuration
HOST = "0.0.0.0"
PORT = 8080

# Connect to Wi-Fi
def connect_wifi():
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)
    wlan.connect(WIFI_SSID, WIFI_PASSWORD)
    
    while not wlan.isconnected():
        print("Connecting to Wi-Fi...")
        time.sleep(1)
    
    print("Connected to Wi-Fi:", wlan.ifconfig())

# Map ADC values to a percentage (0-100%)
def map_adc_value(value, istart=0, istop=4095, ostart=0, ostop=100):
    scaled = ostart + (ostop - ostart) * ((value - istart) / (istop - istart))
    return max(ostart, min(ostop, int(scaled)))

# Send humidity data via HTTP POST
def send_humidity_data(sensor_id, moisture):
    url = "http://localhost:5000/plant_humidity"  # Change to actual API endpoint
    payload = {"sensor_id": sensor_id, "humidity": moisture}
    
    try:
        response = urequests.post(url, json=payload)
        print("Sent data to server. Response:", response.status_code)
        response.close()
    except Exception as e:
        print("Failed to send data:", e)

# Handle incoming HTTP requests
def handle_request(client, valve):
    request = client.recv(1024).decode("utf-8")
    print("Request received:", request)
    
    try:
        # Extract JSON body from HTTP request
        body_start = request.find("\r\n\r\n")
        if body_start != -1:
            body = request[body_start + 4:].strip()
            command = json.loads(body)

            if command.get("command") == "activate_valve":
                valve_id = command.get("valve_id")
                duration = command.get("duration")

                if valve_id == 2 and duration:
                    print(f"Activating valve {valve_id} for {duration} seconds")
                    valve.on()
                    time.sleep(duration)
                    valve.off()

                    response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n" + json.dumps({"message": "Valve activated successfully"})
                    client.send(response.encode())
                    client.close()
                    return

    except Exception as e:
        print("Error processing request:", e)

    error_response = "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\n\r\n" + json.dumps({"error": "Invalid request"})
    client.send(error_response.encode())
    client.close()

# Main function
def main():
    connect_wifi()

    # Initialize ADC sensors
    adc1 = machine.ADC(machine.Pin(ADC_PIN_1))
    adc1.atten(machine.ADC.ATTN_11DB)  # Set attenuation for full range
    adc2 = machine.ADC(machine.Pin(ADC_PIN_2))
    adc2.atten(machine.ADC.ATTN_11DB)

    # Initialize Valve GPIO
    valve = machine.Pin(VALVE_PIN, machine.Pin.OUT)

    # Start HTTP Server
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind((HOST, PORT))
    server.listen(5)
    print(f"Server listening on {HOST}:{PORT}")

    while True:
        try:
            client, addr = server.accept()
            print("Client connected:", addr)
            handle_request(client, valve)
        except Exception as e:
            print("Connection error:", e)

# Run main function
if __name__ == "__main__":
    main()
