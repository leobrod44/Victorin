import RPi.GPIO as GPIO
# getting the time libraly
import time

# setting a current mode
GPIO.setmode(GPIO.BCM)
#removing the warings 
GPIO.setwarnings(False)
pins = [14,15,17,18] 
#setting the mode for all pins so all will be switched on 
GPIO.setup(pins, GPIO.OUT)

for pin in pins:
    GPIO.output(pin, 1)

    