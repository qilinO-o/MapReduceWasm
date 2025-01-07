#include "reducer.h"
#include <stdlib.h>
#include <string.h>

void exports_component_reducewc_reduce_reduce(reducer_string_t *key, reducer_list_string_t *values, reducer_tuple2_string_string_t *ret) {
    uint8_t *key_buffer = (uint8_t *)malloc(key->len + 1);
    *(key_buffer + key->len) = '\0';
    memcpy(key_buffer, key->ptr, key->len);
    reducer_string_set(&(ret->f0), (const char *)key_buffer);
    uint8_t *buffer = (uint8_t *)malloc(20);
    uint8_t *ptr = buffer + 20 - 1;
    *ptr = '\0';
    size_t value = values->len;
    size_t slen = 0;
    if (value == 0) {
        *(--ptr) = '0';
        slen = 1;
    } else {
        while (value > 0) {
            *(--ptr) = '0' + (value % 10);
            value /= 10;
            slen += 1;
        }
    }
    reducer_string_set(&(ret->f1), (const char *)ptr);
}