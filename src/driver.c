// this file is a runtime for program starting with `entry_point`
#include <stdio.h>

extern int entry_point();

int main() {
    printf("%d\n", entry_point());
}
