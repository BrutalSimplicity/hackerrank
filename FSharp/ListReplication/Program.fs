// Learn more about F# at http://fsharp.org

open System

let replicate num ls =
    ls |> List.collect (List.replicate num)

let readList() =
    let rec readListRec ls =
        let input = Console.ReadLine()
        match input with
        | null | "" -> ls
        | x -> readListRec ((int x)::ls)

    [] |> readListRec |> List.rev

[<EntryPoint>]
let main argv =
    let numReplications = Console.ReadLine() |> int
    let replicateN = replicate numReplications

    for x in replicateN (readList()) do
        printfn "%d" x

    0 // return an integer exit code
