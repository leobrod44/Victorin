import RPi.GPIO as GPIO
# getting the time libraly
import time

# setting a current mode
GPIO.setmode(GPIO.BCM)
#removing the warings 
GPIO.setwarnings(False)
#creating a list (array) with the number of GPIO's that we use 
pins = [22, 27] 

#setting the mode for all pins so all will be switched on 
GPIO.setup(pins, GPIO.OUT)

while True:
    for pin in pins:
        GPIO.output(pin,  1)
        time.sleep(1)
    for pin in pins:
        GPIO.output(pin,  0)
        time.sleep(1)
      