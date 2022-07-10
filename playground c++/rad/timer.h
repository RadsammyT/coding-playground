#include <time.h>
// clock can only get time in thousandths/0.001's
namespace timer {

    class Timer {
        clock_t timer = -1;
        // doing timer != -1 is flawed because it would only work if the timer wasn't started at all since it's initialization.

    public:
        bool isStarted = false; 

        Timer(){}


            void startTimer() {
                this->timer = clock();
                this->isStarted = true;
            }

            void endTimer() {
                if(this->isStarted) {
                    this->timer = clock() - this->timer;
                    this->isStarted = false;
                } else {
                    printf("Timer not started\n");
                }
            }

            float getTime() {
                return (float) this->timer/CLOCKS_PER_SEC;
            }
            void printTime(bool newLine = true) {
                if(newLine)
                    printf("%.3f\n", this->getTime());
                else
                    printf("%.3f", this->getTime());
            }

            
    };
}