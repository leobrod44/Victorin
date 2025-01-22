import RPi.GPIO as GPIO
# getting the time libraly
import time

def main():
    # getting the main GPIO libraly

    # setting a current mode
    GPIO.setmode(GPIO.BCM)
    #removing the warings 
    GPIO.setwarnings(False)
    #creating a list (array) with the number of GPIO's that we use 
    pins = [18,17,15,14, 22, 27] 

    #setting the mode for all pins so all will be switched on 
    GPIO.setup(pins, GPIO.OUT)

    while True:
        GPIO.output(14,  GPIO.HIGH)
        GPIO.output(22,  GPIO.HIGH)
        GPIO.output(27,  GPIO.HIGH)
        #wait 0,5 second
        time.sleep(2)

        GPIO.output(14,  GPIO.LOW)
        GPIO.output(22,  GPIO.LOW)
        GPIO.output(27,  GPIO.LOW)

        time.sleep(2)


if __name__ == "__main__":
    main()
