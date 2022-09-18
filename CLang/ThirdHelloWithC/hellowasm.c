#include <stdio.h>
#include <emscripten/emscripten.h>


#ifdef __cplusplus
    #define EXTERN extern "C"
#else
    #define EXTERN
#endif

EXTERN EMSCRIPTEN_KEEPALIVE void printName( char* name ) {
    printf("%s\n", name);
}