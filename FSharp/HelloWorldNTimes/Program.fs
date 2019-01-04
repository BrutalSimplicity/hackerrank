// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    let numTimes = Console.ReadLine() |> int
    for _ in [1..numTimes] do
        printfn "Hello World"
    0 // return an integer exit code
