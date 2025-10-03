#include <atomic>
#include <chrono>
#include <cstdio>
#include <ostream>
#include <thread>
#include <iostream>

#include <pigpio.h>
#include "libtdtp.h"

#define MPSC_CHANNEL_SIZE 8192
#define GPIO_BCM_PIN 17

void *sender;
std::atomic_bool running{true};

void gpioInterrupt(int pin, int level, uint32_t tick) {
    if (level != 0) return;

    c_server_channel_send(0xdeadbeef, sender);
}

void sig_handler(int signum) {
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
    ChannelPair pair = c_server_channel(MPSC_CHANNEL_SIZE);
    sender = pair.tx;

    int res = 0;

    if (registerHandler() != 0) {
        std::cerr << "GPIO init failed" << std::endl;
        res = 1;
    }

    if (c_server(127, 0, 0, 1, 25565, pair.rx) != 0) {
        std::cerr << "failed to start server" << std::endl;
        perror("const char *s");
        res = 1;
    };

    c_free_server_sender(sender);
    gpioTerminate();
    return res;
}
