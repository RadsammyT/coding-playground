#include <time.h>

namespace rad {

    class Timer {
        clock_t timer = -1;

        public:
            Timer(){
                
            }


            void startTimer()
            {
                this->timer = clock();
            }

            void endTimer() {
                if(this->timer != -1) {
                    this->timer = clock() - this->timer;
                } else {
                    printf("Timer not started\n");
                }
            }

            float getTime() {
                return (float) this->timer/1000;
            }
            void printTime() {
                printf("%.3f", getTime());
            }
    };
}