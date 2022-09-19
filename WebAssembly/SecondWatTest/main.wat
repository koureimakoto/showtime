(module
    (func $i (import "imports" "firstWatFn") (param i32))
    ;; First Exported Function writed in Wat
    (func (export "expFirstWatFn") 
        i32.const 42
        call $i
    )
    ;; Second Exported Function writed in Wat
    (func (export "extSecondWatFn") 
        i32.const 51
        call $i
    )
)
