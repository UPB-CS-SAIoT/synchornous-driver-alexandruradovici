/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include <tock.h>
#include <hello.h>

int main(void) {
  if (driver_exists (DRIVER_NUM_HELLO)) {
    printf("Hello driver is present\n");
    hello_print();
  }
  else
  {
    printf("Hello driver is not present\n");
  }
  return 0;
}
