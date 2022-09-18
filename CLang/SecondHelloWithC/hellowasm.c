/*
 * emcc -o hellowasm.html hellowasm.c -s NO_EXIT_RUNTIME=1 -s "EXPORTED_RUNTIME_METHODS=['ccall']" 
 */

#include <stdio.h>
#include <emscripten/emscripten.h>


int
main() {
    printf("Second Hello Wasm\n");
    return 0;
}


#ifdef __cplusplus
    #define EXTERN extern "C"
#else
    #define EXTERN
#endif

EXTERN EMSCRIPTEN_KEEPALIVE int myFn( int argc, char ** argv ) {
    printf("My Function Called\n");
    return 0;
}