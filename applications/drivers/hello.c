#include "tock.h"
#include "hello.h"

bool hello_print(void) {
    syscall_return_t r = command(DRIVER_NUM_HELLO, 1, 0, 0);
    return r.type == TOCK_SYSCALL_SUCCESS;
}