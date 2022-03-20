// this file is a runtime for program starting with `entry_point`
#include <stdio.h>

extern int entry_point();

#define BOOLEAN_MASK        0b01111111
#define BOOLEAN_TAG         0b00011111
#define BOOLEAN_SHIFT       7

#define CHARACTER_MASK      0b11111111
#define CHARACTER_TAG       0b00001111
#define CHARACTER_SHIFT     8

#define FIXNUM_MASK         0b00000011
#define FIXNUM_TAG          0b00000000
#define FIXNUM_SHIFT        2

#define EMPTY_LIST          0b00101111

typedef enum {
    TYPE_BOOLEAN,
    TYPE_CHARACTER,
    TYPE_FIXNUM,
    TYPE_LIST,
} type_t;

typedef struct {
    int val;
    type_t type;
} typed_val_t;

typed_val_t parse_type_info(int val) {
    if ((val & CHARACTER_MASK) == CHARACTER_TAG) {
        return (typed_val_t) {val >> CHARACTER_SHIFT, TYPE_CHARACTER};
    }

    if ((val & BOOLEAN_MASK) == BOOLEAN_TAG) {
        return (typed_val_t) {val >> BOOLEAN_SHIFT, TYPE_BOOLEAN};
    }

    if ((val & FIXNUM_MASK) == FIXNUM_TAG) {
        return (typed_val_t) {val >> FIXNUM_SHIFT, TYPE_FIXNUM};
    }

    if (val == EMPTY_LIST) {
        return (typed_val_t) {EMPTY_LIST, TYPE_LIST};
    }

    return (typed_val_t) {-1, -1};
}

int main() {
    int val = entry_point();
    typed_val_t typed_val = parse_type_info(val);

    switch (typed_val.type) {
        case TYPE_BOOLEAN:
            printf("%s\n", typed_val.val == 1 ? "#t" : "#f");
            break;

        case TYPE_CHARACTER:
            printf("#\\%c\n", typed_val.val);
            break;

        case TYPE_FIXNUM:
            printf("%d\n", typed_val.val);
            break;

        case TYPE_LIST:
            printf("()\n");
            break;

        default:
            printf("unknown type tag %d!\n", typed_val.type);
            break;
    }
}
