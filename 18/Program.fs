open System
open System.IO

type Point = int * int * int
type Area = Set<Point>

let newPoint = function
    | [|x; y; z|] -> Point(x, y, z)
    | _ -> failwith "invalid point input"

let buildPoints (area) (ln:string) =
    Set.add (ln.Split ','
             |> Array.map Int32.Parse
             |> newPoint) area

let countFreeSides area point =
    let (x, y, z) = point
    [(x+1, y, z); (x-1, y, z); (x, y+1, z); (x, y-1, z); (x, y, z+1); (x, y, z-1)]
        |> List.filter (fun p -> Set.contains p area |> not)
        |> List.length

let surfaceArea area =
    area
        |> Set.toSeq
        |> Seq.map (countFreeSides area)
        |> Seq.sum

let fromFile name =
    IO.File.ReadLines(name)
         |> Seq.fold buildPoints (Area [])

[<EntryPoint>]
let main args =
    let name =
        match args with
         | [| s |] -> s
         | _ -> failwith "expected a single input file argument"

    let area = fromFile name
    surfaceArea area |> printfn "[PART ONE]: %d"
    0
