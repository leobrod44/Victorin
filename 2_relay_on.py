import RPi.GPIO as GPIO
# getting the time libraly
import time

# setting a current mode
GPIO.setmode(GPIO.BCM)
#removing the warings 
GPIO.setwarnings(False)


#setting the mode for all pins so all will be switched on 
GPIO.setup([22,27], GPIO.OUT)

GPIO.output(22,  GPIO.HIGH)

GPIO.output(27,  GPIO.HIGH)