open Printf
open Core.List

let () =
    [1; 1; 1]
    |> folding_map ~init:0 ~f:(fun acc x -> (x+acc, x+acc))
    |> iter ~f:(printf "%d\n")
