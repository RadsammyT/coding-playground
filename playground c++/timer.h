#include <time.h>
// clock can only get time in thousands
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