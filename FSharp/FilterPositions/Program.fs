open System

let readLine() = Console.ReadLine()
let hasInput = String.IsNullOrWhiteSpace >> not
let isEven n = n % 2 = 0
let lines =
    Seq.initInfinite (fun i -> (i + 1, readLine()))
    |> Seq.takeWhile (snd >> hasInput)
    |> Seq.toList

lines
|> List.filter (fst >> isEven)
|> List.iter (snd >> printfn "%s")
