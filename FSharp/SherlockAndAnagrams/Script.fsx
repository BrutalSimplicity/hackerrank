open System

let sherlockAnagram (input: string) =
    let substrings (str: string) = seq { for i in [0..str.Length - 1] do
                                         for j in [i..str.Length - 1] ->
                                            str.[i..j] }

    let sortString: string -> string
        = Seq.sort >> String.Concat

    let countSubstringPairs n = n * (n - 1) / 2

    substrings input
    |> Seq.countBy sortString
    |> Seq.map snd
    |> Seq.sumBy countSubstringPairs

let sortString (s: string) =
    let chars = s.ToCharArray()
    let sorted = chars |> Array.sort
    new string(sorted)

