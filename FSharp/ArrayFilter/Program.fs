open System

let hasInput s = not (String.IsNullOrWhiteSpace(s))
let readLine() = Console.ReadLine()

let limit = readLine() |> int
let isValid x = x < limit

Seq.initInfinite (fun _ -> readLine())
|> Seq.takeWhile hasInput
|> Seq.map int
|> Seq.where isValid
|> Seq.iter (printfn "%d")
