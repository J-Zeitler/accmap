let () =
    [1; 1; 1]
    |> List.scan (fun acc x -> acc + x) 0
    |> List.tail
    |> printfn "%A"
