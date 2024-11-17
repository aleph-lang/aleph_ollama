let rec syracuse n =
  if n <= 0 then
    failwith "Le nombre doit être strictement positif"
  else if n = 1 then
    [1]
  else if n mod 2 = 0 then
    n :: syracuse (n / 2)
  else
    n :: syracuse (3 * n + 1)

let () =
  let n = 7 in
  let suite = syracuse n in
  Printf.printf "Syracuse for %d : " n;
  List.iter (fun x -> Printf.printf "%d " x) suite;
  print_newline ()

