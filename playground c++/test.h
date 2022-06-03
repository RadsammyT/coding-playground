
namespace rad {

    int test() {
        printf("test");
        return 0;
    }

    namespace wtf { // nested namespaces. oh the humanity
        int what() {
            printf("wtf");
            return 0;
        }
    }

    class testClass {
        int t = 0;

    public:
        int test(){
            printf("testClass");
            return 0;
        }

        int getT() {
            return t;
        }
        int setT(int in) {
            this->t = in;
            return 0;
        }
    };
    

}