solve :: [Int] -> Int
solve report = sum $ zipWith (\x y -> if y > x then 1 else 0) report (tail report)

solve2 :: [Int] -> Int
solve2 report =
  let sums = zipWith3 (\x y z -> x + y + z) report (tail report) (tail $ tail report)
   in sum $ zipWith (\x y -> if y > x then 1 else 0) sums (tail sums)

main :: IO ()
main = do
  report <- map read . lines <$> readFile "inputs/day1.txt"
  putStr "The answer to part one is: "
  print $ solve report

  putStr "The answer to part two is: "
  print $ solve2 report
