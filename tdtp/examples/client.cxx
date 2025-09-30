#include "libtdtp.h"
#include <cstdio>
#include <iostream>
#include <iomanip>
#include <thread>

void print_hex_u128(unsigned __int128 x) {
    uint64_t hi = static_cast<uint64_t>(x >> 64);
    uint64_t lo = static_cast<uint64_t>(x);
    if (hi)
        std::cout << "0x" << std::hex << hi << std::setfill('0') << std::setw(16) << lo;
    else
        std::cout << "0x" << std::hex << lo;
}

void packet_handler(void *receiver, int *result) {
    std::cout << "Hello from handler thread\n";

    int count = 0;

    while (count <= 20) {
        IncomingDataPacket out;
        if (c_client_channel_recv(&out, receiver)) {
            print_hex_u128(out);
            std::cout << "\n";
        } else {
            *result = 1;
            return;
        }
    }

    *result = 0;
    return;
}

int main() {
    std::cout << "Hello, initializing channel.\n";
    ChannelPair pair = c_client_channel(8192);
    void *rx = pair.rx;
    void *tx = pair.tx;
    std::cout << "Initialized channels.\n";

    int handler_result = 0;
    std::thread handler(packet_handler, rx, &handler_result);
    std::cout << "Started handler thread.\n";

    std::cout << "Starting connection...\n";
    if (c_data(127, 0, 0, 1, 8888, tx) != 0) {
        perror("oops, error\n");
        handler.join();
        return 1;
    }

    if (handler_result != 0) {
        std::cerr << "handler got an error, fuck";
    }
}
