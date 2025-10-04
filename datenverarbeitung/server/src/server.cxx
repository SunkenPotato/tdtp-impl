#include <atomic>
#include <chrono>
#include <cstdio>
#include <iostream>
#include <ostream>
#include <signal.h>
#include <thread>

#include <pigpio.h>
#include "../../include/libtdtp.h"

#define MPSC_CHANNEL_SIZE 8192
#define GPIO_BCM_PIN 17

void *sender;
std::atomic_bool running{true};

std::chrono::system_clock::time_point prog_start;

void gpioAlert(int gpio, int level, uint32_t tick) {
    // falling edge && sender has not been dropped by signal handler
    if (level == 0 && running.load()) {
        // get duration elapsed since we started the program.
        auto elapsed = std::chrono::high_resolution_clock::now() - prog_start;
        OutgoingDataPacket microsecs = std::chrono::duration_cast<std::chrono::microseconds>(elapsed).count();
        // send that data along...
        c_server_channel_send(microsecs, sender);
    }
}

void sig_handler(int signum) {
    // make sure to let the alert function know that we've received a Ctrl-C signal the pointer is no longer valid!
    running.store(false);
    // when this is freed/dropped, the server will return and the program will terminate, since this is blocking.
    c_free_server_sender(sender);
}

int registerHandler() {
    // initialise the library and simultaneously become pigpiod
    if (gpioInitialise() < 0) {
        return 1;
    }

    gpioSetMode(GPIO_BCM_PIN, PI_INPUT);
    gpioSetAlertFunc(GPIO_BCM_PIN, gpioAlert);

    return 0;
}

int main() {
    // initialise the logging framework with verbosity set to "info" (3).
    init_logger_framework(3);
    // record the start of the program
    prog_start = std::chrono::high_resolution_clock::now();

    // register a signal handler, since this will never really terminate, unless we hit Ctrl-C or the client disconnects
    // TODO, client disconnection bug
    signal(SIGINT, sig_handler);

    // create a channel pair for TX/RX
    ChannelPair pair = c_server_channel(MPSC_CHANNEL_SIZE);
    sender = pair.tx;

    int res = 0;

    // register the GPIO alert function/handler
    if (registerHandler() < 0) {
        std::cerr << "GPIO init failed" << std::endl;
        res = 1;
    }

    // this will block, so no need to block with a `while`-loop.
    //
    // -2 = channel dropped // voluntary termination
    std::cout << "Starting server" << std::endl;
    if (c_server(127, 0, 0, 1, 25565, pair.rx) != -2) {
        perror("c_server");
        res = 1;
    };

    std::cout << "Cleaning up and exiting" << std::endl;
    gpioTerminate();
    return res;
}
