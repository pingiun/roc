app "rbtree-del"
    packages { base: "platform" }
    imports [base.Task]
    provides [ main ] to base


Color : [ Red, Black ]

Tree a b : [ Leaf, Node Color (Tree a b) a b (Tree a b) ]

Map : Tree I64 Bool

ConsList a : [ Nil, Cons a (ConsList a) ]

main : Task.Task {} []
main =
    # benchmarks use 4_200_000
    m = makeMap 420

    val = fold (\_, v, r -> if v then r + 1 else r) m 0

    val
        |> Str.fromInt
        |> Task.putLine

boom : Str -> a
boom = \_ -> boom ""

makeMap : I64 -> Map
makeMap = \n ->
    makeMapHelp n n Leaf

makeMapHelp : I64, I64, Map -> Map
makeMapHelp = \total, n, m ->
    when n is
        0 -> m
        _ ->
            n1 = n - 1

            powerOf10 =
                (n % 10 |> resultWithDefault 0) == 0

            t1 = insert m n powerOf10

            isFrequency =
                (n % 4 |> resultWithDefault 0) == 0

            key = n1 + ((total - n1) // 5 |> resultWithDefault 0)
            t2 = if isFrequency then delete t1 key else t1

            makeMapHelp total n1 t2

fold : (a, b, omega -> omega), Tree a b, omega -> omega
fold = \f, tree, b ->
    when tree is
        Leaf  -> b
        Node _ l k v r -> fold f r (f k v (fold f l b))

depth : Tree * * -> I64
depth = \tree ->
    when tree is
        Leaf -> 1
        Node _ l _ _ r -> 1 + depth l + depth r

resultWithDefault : Result a e, a -> a
resultWithDefault = \res, default ->
    when res is
        Ok v -> v
        Err _ -> default


insert : Map, I64, Bool -> Map
insert = \t, k, v -> if isRed t then setBlack (ins t k v) else ins t k v


setBlack : Tree a b -> Tree a b
setBlack = \tree ->
    when tree is
        Node _ l k v r -> Node Black l k v r
        _ -> tree

isRed : Tree a b -> Bool
isRed = \tree ->
    when tree is
        Node Red _ _ _ _ -> True
        _ -> False

lt = \x, y -> x < y

ins : Map, I64, Bool -> Map
ins = \tree, kx, vx ->
    when tree is
        Leaf ->
            Node Red Leaf kx vx Leaf

        Node Red a ky vy b ->
            if lt kx ky then
                Node Red (ins a kx vx) ky vy b
            else if lt ky kx then
                Node Red a ky vy (ins b kx vx)
            else
                Node Red a ky vy (ins b kx vx)

        Node Black a ky vy b ->
            if lt kx ky then
              (if isRed a then balanceLeft (ins a kx vx) ky vy b else Node Black (ins a kx vx) ky vy b)
            else if lt ky kx then
              (if isRed b then balanceRight a ky vy (ins b kx vx) else Node Black a ky vy (ins b kx vx))
            else Node Black a kx vx b

balanceLeft : Tree a b, a, b, Tree a b -> Tree a b
balanceLeft = \l, k, v, r ->
    when l is
      Leaf -> Leaf
      Node _  (Node Red lx kx vx rx) ky vy ry
        -> Node Red (Node Black lx kx vx rx) ky vy (Node Black ry k v r)
      Node _ ly ky vy (Node Red lx kx vx rx)
        -> Node Red (Node Black ly ky vy lx) kx vx (Node Black rx k v r)
      Node _ lx kx vx rx
        -> Node Black (Node Red lx kx vx rx) k v r

balanceRight : Tree a b, a, b, Tree a b -> Tree a b
balanceRight = \l, k, v, r ->
    when r is
      Leaf -> Leaf
      Node _ (Node Red lx kx vx rx) ky vy ry
        -> Node Red (Node Black l k v lx) kx vx (Node Black rx ky vy ry)
      Node _ lx kx vx (Node Red ly ky vy ry)
        -> Node Red (Node Black l k v lx) kx vx (Node Black ly ky vy ry)
      Node _ lx kx vx rx
        -> Node Black l k v (Node Red lx kx vx rx)

isBlack : Color -> Bool
isBlack = \c ->
    when c is
        Black -> True
        Red -> False


Del a b : [ Del (Tree a b) Bool  ]

setRed : Map -> Map
setRed = \t ->
    when t is
      Node _ l k v r -> Node Red l k v r
      _ -> t



makeBlack : Map -> Del I64 Bool
makeBlack = \t ->
    when t is
      Node Red l k v r -> Del (Node Black l k v r) False
      _                -> Del t True


rebalanceLeft = \c, l, k, v, r ->
  when l is
      Node Black _ _ _ _   -> Del (balanceLeft (setRed l) k v r) (isBlack c)
      Node Red lx kx vx rx -> Del (Node Black lx kx vx (balanceLeft (setRed rx) k v r)) False
      _ -> boom "unreachable"

rebalanceRight  = \c, l, k, v, r ->
  when r is
      Node Black _ _ _ _   -> Del (balanceRight l k v (setRed r))  (isBlack c)
      Node Red lx kx vx rx -> Del (Node Black (balanceRight l k v (setRed lx)) kx vx rx) False
      _ -> boom "unreachable"



delMin = \t ->
    when t is
        Node Black Leaf k v r ->
            when r is
                Leaf -> Delmin (Del Leaf True) k v
                _    -> Delmin (Del (setBlack r) False) k v

        Node Red Leaf k v r ->
            Delmin (Del r False) k v

        Node c l k v r ->
            when delMin l is
                Delmin (Del lx True) kx vx  -> Delmin (rebalanceRight c lx k v r) kx vx
                Delmin (Del lx False) kx vx -> Delmin (Del (Node c lx k v r) False) kx vx

        Leaf ->
            Delmin (Del t False) 0 False



delete : Map, I64 -> Map
delete = \t, k ->
    when del t k is
      Del tx _ -> setBlack tx

del = \t, k ->
    when t is
      Leaf -> Del Leaf False
      Node cx lx kx vx rx ->
        if (k < kx) then
            when (del lx k) is
                Del ly True  -> rebalanceRight cx ly kx vx rx
                Del ly False -> Del (Node cx ly kx vx rx) False

        else if (k > kx) then
            when (del rx k) is
                Del ry True  -> rebalanceLeft cx lx kx vx ry
                Del ry False -> Del (Node cx lx kx vx ry) False

        else
            when rx is
                Leaf -> if isBlack cx then makeBlack lx else Del lx False
                Node _ _ _ _ _    ->
                    when delMin rx is
                          Delmin (Del ry True) ky vy  -> rebalanceLeft cx lx ky vy ry
                          Delmin (Del ry False) ky vy -> Del (Node cx lx ky vy ry) False