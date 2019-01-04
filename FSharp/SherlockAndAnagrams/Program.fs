open System

let sherlockAnagram (input: string) =
    let substrings (str: string) = seq { for i in [0..str.Length - 1] do
                                         for j in [i..str.Length - 1] ->
                                            str.[i..j] }

    let sortString (s: string) = s |> Seq.sort |> String.Concat

    let countSubstringPairs n = n * (n - 1) / 2

    substrings input
    |> Seq.countBy sortString
    |> Seq.map snd
    |> Seq.sumBy countSubstringPairs

let numQueries = Console.ReadLine() |> Convert.ToInt32

for _ in [1..numQueries] do
    Console.ReadLine()
    |> sherlockAnagram
    |> printfn "%i"
