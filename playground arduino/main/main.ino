#include <TimeLib.h>
#include <DS1307RTC.h>

#include <DS3231.h>
#include <Wire.h>
#include "test.h"
#include <LiquidCrystal.h>

#pragma once
bool setTimer = false;
void setup() {
    if(setTimer)
        setDS3231Time::setup();
    else
        lcd_clock::setup();
}


void loop() {
    if(!setTimer)
        lcd_clock::loop();
}







