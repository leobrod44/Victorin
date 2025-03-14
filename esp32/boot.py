import network
import socket
import ujson as json
import urequests
import machine
import time
import _thread
import sys

# Wi-Fi Configuration
WIFI_SSID = "Balais"
WIFI_PASSWORD = "dUst102!"

# Devices Configuration
DEVICES = {
    1: {"valve": 15, "sensor": 32, "min_adc": 1250, "max_adc": 2380},  # Device 1
    2: {"valve": 18, "sensor": 33, "min_adc": 1250, "max_adc": 2380}    # Device 2
}

PUMP_PIN = 27

# Initialize ADC and Valves for all devices
for device_key, device in DEVICES.items():
    device["valve_pin"] = machine.Pin(device["valve"], machine.Pin.OUT)  # Valve Pin object
    device["adc"] = machine.ADC(machine.Pin(device["sensor"]))  # ADC Sensor
    device["adc"].atten(machine.ADC.ATTN_11DB)  # Set attenuation for 0-3.9V range
    device["readings"] = []  # Store readings for averaging

# Pump Initialization
pump = machine.Pin(PUMP_PIN, machine.Pin.OUT)
pump.value(1)

ESP32_GPIO_PINS = [15, 18,27]
# Initialize pins
pins = [machine.Pin(pin_num, machine.Pin.OUT) for pin_num in ESP32_GPIO_PINS]

for pin in pins:
    pin.value(0)


    print("Cycle complete")
# Initialize all pins as output


# Connect to Wi-Fi
def connect_wifi():
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)
    wlan.connect(WIFI_SSID, WIFI_PASSWORD)
    
    while not wlan.isconnected():
        print("Connecting to Wi-Fi...")
        time.sleep(1)
    
    print("Connected to Wi-Fi:", wlan.ifconfig())

# Map ADC values to 0-100% range
def map_adc_value(adc_value, min_adc, max_adc):
    moisture_percentage = max(0, min(100, 100 - ((adc_value - min_adc) / (max_adc - min_adc) * 100)))
    return moisture_percentage

# Send humidity data
def send_humidity_data():
    url = "http://192.168.0.15:3031/humidity_plant"  # Replace with actual API
    
    while True:
        for device_key, device in DEVICES.items():
            adc = device["adc"]
            min_adc = device["min_adc"]
            max_adc = device["max_adc"]
            
            moisture = map_adc_value(adc.read(), min_adc, max_adc)
            
            # Keep last 5 readings for averaging
            device["readings"].append(moisture)
            if len(device["readings"]) > 5:
                device["readings"].pop(0)

            avg_moisture = sum(device["readings"]) / len(device["readings"])
            
            #print(f"Device {device_key} -> Sensor {device['sensor']} -> Moisture {avg_moisture}% (Raw: {adc.read()})")
            
            payload = {"id": device_key, "humidity": avg_moisture}
            try:
                response = urequests.post(url, json=payload)
                response.close()
               # print(f"Sent humidity data: {payload}")
            except Exception as e:
                print("Failed to send humidity data:", e)
        
        time.sleep(1)

def activate_valve_async(valve_gpio, duration, client):
    try:
        for device_key, device in DEVICES.items():
            if device["valve"] == valve_gpio:
                valve = device["valve_pin"]
                print(f"Activating valve {valve_gpio} for {duration} seconds")
                valve.value(1)
                time.sleep(duration)
                valve.value(0)
                print(f"Deactivated valve {valve_gpio}")
                
                cycle_url = "http://192.168.0.15:3031/cycle_complete"
                urequests.post(cycle_url, json={"device_id": device_key})
                print(f"Cycle complete for valve {device_key}")
                
                response = json.dumps({"message": "Valve activated successfully"})
                
                # Send response back in the main thread
                _thread.start_new_thread(send_response, (client, response))
                return
        
        print(f"No valve found for GPIO {valve_gpio}")
    
    except Exception as e:
        print(f"An error occurred: {e}")
        # Ensure to close the socket if it's open
        if client.fileno() != -1:
            _thread.start_new_thread(send_response, (client, json.dumps({"error": str(e)})))
            client.close()

# Function to send response back to the client
def send_response(client, response):
    try:
        if client.fileno() != -1:
            client.send("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n" + response)
            print("Response sent")
    except Exception as e:
        print(f"Error sending response: {e}", file=sys.stderr)
    finally:
        # Make sure to close the client socket after sending response
        try:
            if client.fileno() != -1:
                client.close()
        except Exception:
            pass

# Handle HTTP Requests
def handle_request(client):
    try:
        request = client.recv(1024).decode("utf-8")
        #print("Request received:", request)

        request_line = request.split("\n")[0]
        path = request_line.split(" ")[1] if len(request_line.split(" ")) > 1 else "/"

        if path.startswith("/"):
            path = path[1:]

        body_start = request.find("\r\n\r\n")
        body = request[body_start + 4:].strip() if body_start != -1 and len(request) > body_start + 4 else ""

        command = {}
        if body:
            try:
                command = json.loads(body)
            except json.JSONDecodeError:
                print("Invalid JSON body, ignoring.")

        print("Path:", path)

        if path == "activate":
            valve_gpio = command.get("device_gpio")
            duration = command.get("duration")
            print(valve_gpio)
            print(duration)
            if valve_gpio is not None and duration:
                _thread.start_new_thread(activate_valve_async, (valve_gpio, duration, client))
                response = json.dumps({"message": f"Valve {valve_gpio} activated"})
                client.send("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n" + response)
        
        elif path == "activate_pump":
            print("Activating pump")
            pump.value(1)
            response = json.dumps({"message": "Pump activated"})
            client.send("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n" + response)
            print("Pump response sent")
        
        elif path == "deactivate_pump":
            #print("Deactivating pump")
            pump.value(0)
            response = json.dumps({"message": "Pump deactivated"})
            client.send("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n" + response)
        
        else:
            raise ValueError("Invalid command")
    
    except Exception as e:
        print(f"Error processing request: {e}", file=sys.stderr)
        error_response = json.dumps({"error": str(e)})
        client.send("HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\n\r\n" + error_response)
    
    finally:
        client.close()

# Main function
def main():
    try:
        connect_wifi()
        _thread.start_new_thread(send_humidity_data, ())
        server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server.bind(("0.0.0.0", 8080))
        server.listen(5)
        print("Server listening on 0.0.0.0:8080")

        while True:
            try:
                client, addr = server.accept()
                print("Client connected:", addr)
                handle_request(client)
            except Exception as e:
                print(f"Connection error: {e}", file=sys.stderr)
    
    except Exception as e:
        print(f"Fatal error: {e}", file=sys.stderr)

# Run main function
if __name__ == "__main__":
    main()

