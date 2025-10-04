#include <atomic>
#include <chrono>
#include <cstdio>
#include <ostream>
#include <thread>
#include <iostream>
#include <signal.h>

#include <pigpio.h>
#include "../include/libtdtp.h"

#define MPSC_CHANNEL_SIZE 8192
#define GPIO_BCM_PIN 17

void *sender;
std::atomic_bool running{true};

void gpioInterrupt(int gpio, int level, uint32_t tick) {
    if (level == 0) {
        std::cout << "Got impulse" << std::endl;
        // c_server_channel_send(0xdeadbeef, sender);
    }
}

void sig_handler(int signum) {
    std::cout << "Got signal " << signum << std::endl;
    running.store(false, std::memory_order_seq_cst);
}

int registerHandler() {
    if (gpioInitialise() < 0) {
        return 1;
    }

    gpioSetMode(GPIO_BCM_PIN, PI_INPUT);
    gpioSetAlertFunc(GPIO_BCM_PIN, gpioInterrupt);

    return 0;
}

int main() {
    std::cout << "Adding signal hook" << std::endl;
    signal(SIGINT, sig_handler);

    std::cout << "Creating channel" << std::endl;
    ChannelPair pair = c_server_channel(MPSC_CHANNEL_SIZE);
    sender = pair.tx;

    int res = 0;

    std::cout << "Registering GPIO handler" << std::endl;
    if (registerHandler() < 0) {
        std::cerr << "GPIO init failed" << std::endl;
        res = 1;
    }

    /*std::cout << "Starting server" << std::endl;
    if (c_server(127, 0, 0, 1, 25565, pair.rx) != 0) {
        std::cerr << "failed to start server" << std::endl;
        perror("const char *s");
        res = 1;
        };*/
    while (running.load(std::memory_order_seq_cst))
        std::this_thread::sleep_for(std::chrono::milliseconds(200));

    std::cout << "Cleaning up and exiting" << std::endl;
    c_free_server_sender(sender);
    gpioTerminate();
    return res;
}
