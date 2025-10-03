#include "libtdtp.h"
#include <thread>
#include <iostream>
#include <chrono>

void packet_producer(void *tx, int *result) {
    int count = 0;

    while (true) {
        std::chrono::time_point now = std::chrono::system_clock::now();
        auto us = std::chrono::duration_cast<std::chrono::microseconds>(now.time_since_epoch()).count();

        unsigned __int128 ts = static_cast<unsigned __int128>(us);

        if (!c_server_channel_send(ts, tx)) {
            *result = 1;
            std::cerr << "channel disconnected\n";
            return;
        }
        count += 1;
        printf("sent %ith packet", count);
    }
}

int main() {
    std::cout << "creating channels\n";
    ChannelPair pair = c_server_channel(8192);
    void *rx = pair.rx;
    void *tx = pair.tx;

    int prod_result = -1;
    std::thread producer(packet_producer, tx, &prod_result);

    std::cout << "starting server\n";
    if (c_server(127, 0, 0, 1, 8888, rx) != 0) {
        perror("oops, server err");
        producer.join();
        return 1;
    }

    std::cout << "joining thread\n";

    producer.join();

    if (prod_result != 1) {
        printf("producer returned an err %i", prod_result);
        return 1;
    }

    std::cout << "ok\n";
}
