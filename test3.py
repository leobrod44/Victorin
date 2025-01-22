import RPi.GPIO as GPIO
# getting the time libraly
import time

# setting a current mode
GPIO.setmode(GPIO.BCM)
#removing the warings 
GPIO.setwarnings(False)
#creating a list (array) with the number of GPIO's that we use 
pins = [27] 

#setting the mode for all pins so all will be switched on 
GPIO.setup(pins, GPIO.OUT)

while True:
        GPIO.output(22,  1)
        GPIO.output(27,  0)
        print("high")
        time.sleep(2)
        GPIO.output(22,  0)
        GPIO.output(27,  0)
        print("low")
        time.sleep(2)
      