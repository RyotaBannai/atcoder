main :: IO ()
main = solve1

-- ABC081A - Placing Marbles
solve1 :: IO ()
solve1 = do
  xs <- getLine
  print . length . filter (== '1') $ xs

-- ABC081B - Shift only
solve2 :: IO ()
solve2 = do
  _ <- getLine
  xs <- map read . words <$> getLine
  print $ run 0 xs
  where
    run n xs
      | all even xs = run (n + 1) (map (`div` 2) xs)
      | otherwise = n
