#include <pigpio.h>
#include <iostream>
#include <atomic>
#include <csignal>
#include <thread>

std::atomic<int> counter{0};
bool running = true;

void count_callback(int gpio, int level, uint32_t tick)
{
    if (level == 0) // Falling edge
    {
        int c = ++counter;
        std::cout << c << std::endl;
    }
}

void signal_handler(int signum)
{
    running = false;
}

int main()
{
    if (gpioInitialise() < 0)
    {
        std::cerr << "pigpio initialization failed" << std::endl;
        return 1;
    }

    // Handle Ctrl+C clean exit
    std::signal(SIGINT, signal_handler);

    int pin = 17; // BCM pin number
    gpioSetMode(pin, PI_INPUT);

    // Register callback on falling edge
    gpioSetAlertFunc(pin, count_callback);

    // Keep the program alive
    while (running)
    {
        std::this_thread::sleep_for(std::chrono::seconds(1));
    }

    gpioTerminate();
    return 0;
}
