#include <LiquidCrystal.h>
#include "setDS3231Time.h"
#pragma once


#define RGB_BLUE 3
#define RGB_GREEN 5
#define RGB_RED 6

namespace powerBlink {
    void setup() {
        pinMode(2, OUTPUT);
        Serial.begin(9600);
    }

    void loop() {
        digitalWrite(2, HIGH);
        Serial.println("On ");
        delay(1000);
        digitalWrite(2, LOW);
        Serial.println("Off ");
        delay(1000);

    }
}

namespace rgb {
    void setup() {
        pinMode(RGB_RED, OUTPUT);
        pinMode(RGB_GREEN, OUTPUT);
        pinMode(RGB_BLUE, OUTPUT);
        digitalWrite(RGB_RED, HIGH);
        digitalWrite(RGB_GREEN, LOW);
        digitalWrite(RGB_BLUE, LOW);

    }
    int redValue;
    int greenValue;
    int blueValue;
    void loop() {
        #define delayTime 1 // fading time between colors

        redValue = 255; // choose a value between 1 and 255 to change the color.
        greenValue = 0;
        blueValue = 0;


        // this is unnecessary as we've either turned on RGB_RED in SETUP
        // or in the previous loop ... regardless, this turns RGB_RED off
        // analogWrite(RGB_RED, 0);
        // delay(1000);

        for (int i = 0; i < 255; i += 1) // fades out red bring green full when i=255
        {
            redValue -= 1;
            greenValue += 1;
            // The following was reversed, counting in the wrong directions
            // analogWrite(RGB_RED, 255 - redValue);
            // analogWrite(RGB_GREEN, 255 - greenValue);
            analogWrite(RGB_RED, redValue);
            analogWrite(RGB_GREEN, greenValue);
            delay(delayTime);
        }

        redValue = 0;
        greenValue = 255;
        blueValue = 0;

        for (int i = 0; i < 255; i += 1) // fades out green bring blue full when i=255
        {
            greenValue -= 1;
            blueValue += 1;
            // The following was reversed, counting in the wrong directions
            // analogWrite(RGB_GREEN, 255 - greenValue);
            // analogWrite(RGB_BLUE, 255 - blueValue);
            analogWrite(RGB_GREEN, greenValue);
            analogWrite(RGB_BLUE, blueValue);
            delay(delayTime);
        }

        redValue = 0;
        greenValue = 0;
        blueValue = 255;

        for (int i = 0; i < 255; i += 1) // fades out blue bring red full when i=255
        {
            // The following code has been rearranged to match the other two similar sections
            blueValue -= 1;
            redValue += 1;
            // The following was reversed, counting in the wrong directions
            // analogWrite(RGB_BLUE, 255 - blueValue);
            // analogWrite(RGB_RED, 255 - redValue);
            analogWrite(RGB_BLUE, blueValue);
            analogWrite(RGB_RED, redValue);
            delay(delayTime);
        }
    }

    

} 
/*
Wiring :
    (Component Pin to MicroController Pin/Breadboard (if power/ground))
    Clock:
        Clock SCL to SCL12
        Clock SDA to SDA20
        Clock VCC to POWER.
        Clock GND to GROUND.
    LCD: 
        (PWM pins are just numbers)
        LCD VSS to Ground
        LCD VDD TO Power
        LCD VO to PWM (or 2 in this case as this controls contrast).
        LCD RS to PWM 7
        LCD RW to ground
        LCD E to PWM 8
        LCD D4 to PWM 9
        LCD D5 to PWM 10
        LCD D6 to PWM 11
        LCD D7 to PWM 12
        LCD A to Power
        LCD K to Ground
    Breadboard:
        Ground to breadboard ground
        5V Power to breadboard power

*/
namespace lcd_clock {
    #define DS3231_I2C_ADDRESS 0x68

    byte decToBcd(byte val)
    {
      return ( (val / 10 * 16) + (val % 10) );
    }

    void writeOnAddress(byte value, int address)
    {
        Wire.beginTransmission(DS3231_I2C_ADDRESS);
        Wire.write(address);
        Wire.write(decToBcd(value));
        Wire.endTransmission();
    }
    //PWM pins to RS, E, D4, D5, D6, D7.
    LiquidCrystal lcd(7, 8, 9, 10, 11, 12); // numbers are PWM pins
    int tpin = 0;
    const double CONTRAST = 3.5;
    DS3231 rtc;
    RTClib lib;
    bool century = false;
    bool h12Flag = true;
    bool pmFlag = true;
    // setting the clock with compiler dates will reset DOW to 0.
    const char* dow[] = {"Sun","Mon", "Tues", "Wed", "Thurs", "Fri", "Sat", "Sun" };
    void setup() {
        // set up the LCD's number of columns and rows:
        lcd.begin(16, 2);
        // Print a message to the LCD.
        pinMode(2, OUTPUT);
        Wire.begin();
        Serial.begin(57600);
        }

    void loop() {
    // This is OK
        analogWrite(2, 1);
        lcd.setCursor(0, 0);
        // top line, year, month, day
        
        lcd.print(dow[rtc.getDoW()]);
        lcd.print("|");
        lcd.print(rtc.getDoW());
        if(rtc.getDoW() >= 7) {
            writeOnAddress(0, 3); // sets DOW to sunday if a full week goes by
        }
        lcd.print(" - ");
        lcd.print(rtc.getYear());
        lcd.print("/");
        lcd.print(rtc.getMonth(century));
        lcd.print("/");
        lcd.print(rtc.getDate());
        // set the cursor to column 0, line 1
        lcd.setCursor(0, 1);
        if (rtc.getHour(h12Flag, pmFlag) > 12 && rtc.getHour(h12Flag, pmFlag) != 12)
            lcd.print(rtc.getHour(h12Flag, pmFlag) - 12);
        else
            lcd.print(rtc.getHour(h12Flag, pmFlag));
        lcd.print(":");
        if (rtc.getMinute() <= 9) {
            lcd.print(0);
        }
        lcd.print(rtc.getMinute());
        lcd.print(".");
        if (rtc.getSecond() <= 9) {
            lcd.print(0);
        }
        lcd.print(rtc.getSecond());
        if(rtc.getHour(h12Flag, pmFlag) >= 12 && rtc.getHour(h12Flag, pmFlag) != 24) // 
            lcd.print("p");
        else if(rtc.getHour(h12Flag, pmFlag) < 12 || rtc.getHour(h12Flag, pmFlag) == 24)
            lcd.print("a");
        lcd.print("    ");
    }
}


