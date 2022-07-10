namespace rad {
    class testClass {

        int test = 0;
        public: 
            int getT() {
                return this->test;
            }

            void setT(int t) {
                this->test = t;
            }
    };
}