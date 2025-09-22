#include <iostream>;
using namespace std;

class I2B{
    public:
        int hallo;
        static int hallowelt() {
            cout << "Hallo Welt" << endl;
        }
};

int main() {
    I2B i2b;
    i2b.hallo = 5;
    I2B::hallowelt();
}