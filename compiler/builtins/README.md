# So you want to add a builtin?

Builtins are the functions and modules that are implicitly imported into every module. Some of them compile down to llvm, others need to be constructed and defined. Making a new builtin means touching many files. Lets make it easy for you and just list out which modules you need to visit to make a builtin. Here is what it takes:

### module/src/symbol.rs

Towards the bottom of a file there is a `define_builtins!` macro being used that takes many modules and function names. The first level (`List`, `Int` ..) is the module name, and the second level is the function or value name (`reverse`, `mod` ..). If you wanted to add a `Int` function called `addTwo` go to `2 Int: "Int" => {` and inside that case add to the bottom `38 INT_ADD_TWO: "addTwo"` (assuming there are 37 existing ones). 

Some of these have `#` inside their name (`first#list`, #lt` ..). This is a trick we are doing to hide implementation details from Roc programmers. To a Roc programmer, a name with `#` in it is invalid, because `#` means everything after it is parsed to a comment. We are constructing these functions manually, so we are circumventing the parsing step and dont have such restrictions. We get to make functions and values with `#` which as a consequence are not accessible to Roc programmers. Roc programmers simply cannot reference them.

But we can use these values and some of these are necessary for implementing builtins. For example, `List.get` returns tags, and it is not easy for us to create tags when composing LLVM. What is easier however, is:
- ..writing `List.#getUnsafe` that has the dangerous signature of `List elem, Int -> elem` in LLVM
- ..writing `List elem, Int -> Result elem [ OutOfBounds ]*` in a type safe way that uses `getUnsafe` internally, only after it checks if the `elem` at `Int` index exists.


### can/src/builtins.rs

Right at the top of this module is a function called `builtin_defs`. All this is doing is mapping the `Symbol` defined in `module/src/symbol.rs` to its implementation. Some of the builtins are quite complex, such as `list_get`, which is complex the reasons mentioned in the previous section; `list_get` returns tags, and it defers to lower-level functions via an if statement.

Lets look at `List.repeat : elem, Int -> List elem`, which is more straight forward, and points directly to its lower level implementation:
```
fn list_repeat(symbol: Symbol, var_store: &mut VarStore) -> Def {
    let elem_var = var_store.fresh();
    let len_var = var_store.fresh();
    let list_var = var_store.fresh();

    let body = RunLowLevel {
        op: LowLevel::ListRepeat,
        args: vec![
            (elem_var, Var(Symbol::ARG_1)),
            (len_var, Var(Symbol::ARG_2)),
        ],
        ret_var: list_var,
    };

    defn(
        symbol,
        vec![(elem_var, Symbol::ARG_1), (len_var, Symbol::ARG_2)],
        var_store,
        body,
        list_var,
    )
}
```
In these builtin definitions you will need to allocate for and list the arguments. For `List.repeat`, the arguments are the `elem_var` and the `len_var`. So in both the `body` and `defn` we list these arguments in a vector, with the `Symobl::ARG_1` adn` Symvol::ARG_2` designating which argument is which.

Since `List.repeat` is implemented entirely as low level functions, its `body` is a `RunLowLevel`, and the `op` is `LowLevel::ListRepeat`. Lets talk about `LowLevel` in the next section.

## Connecting the definition to the implementation
### module/src/low_level.rs
This `LowLevel` thing connects the builtin defined in this module to its implementation. Its referenced  in `can/src/builtins.rs` and it is used in `gen/src/llvm/build.rs`.

## Bottom level LLVM values and functions
### gen/src/llvm/build.rs
This is where bottom-level functions that need to be written as LLVM are created. If the function leads to a tag thats a good sign it should not be written here in `build.rs`. If its simple fundamental stuff like `INT_ADD` then it certainly should be written here.

## Letting the compiler know these functions exist
Its one thing to actually write these functions, its _another_ thing to let the Roc compiler know they exist as part of the standard library. You have to tell the compiler "Hey, this function exists, and it has this type signature". That happens in these modules:
### builtins/src/std.rs
### builtins/src/unique.rs