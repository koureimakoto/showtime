(module
    (func $i (import "imports" "firstWatFn") (param i32))
    (func (export "expFirstWatFn") 
        i32.const 42
        call $i
    )
)