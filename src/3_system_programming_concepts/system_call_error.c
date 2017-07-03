#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>

int main(void)
{
  int fd = open("./system_call_error.c", O_WRONLY);
  if (fd == -1) {
    perror("./system_call_error.c");
    exit(EXIT_FAILURE);
  }
}
