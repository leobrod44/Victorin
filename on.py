import RPi.GPIO as GPIO
# getting the time libraly
import time

# setting a current mode
GPIO.setmode(GPIO.BCM)
#removing the warings 
GPIO.setwarnings(False)
#creating a list (array) with the number of GPIO's that we use 

pins = [18, 17, 15] 
# in1 18
# in2 17
# in3 15
# in4 14


#setting the mode for all pins so all will be switched on 
GPIO.setup(pins, GPIO.OUT)

for pin in pins:
    GPIO.output(pin, 0)

      