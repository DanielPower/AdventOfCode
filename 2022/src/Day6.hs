module Day6
  ( part1,
    part2,
  )
where

dropUntilDuplicate :: Char -> String -> String
dropUntilDuplicate c "" = ""
dropUntilDuplicate c s
  | c == last s = init s
  | otherwise = dropUntilDuplicate c (init s)

countUntilUnique :: Int -> Int -> String -> String -> Int
countUntilUnique n i s acc
  | length acc == n = i
  | item `elem` acc = countUntilUnique n (i + 1) (tail s) (item : dropUntilDuplicate item acc)
  | otherwise = countUntilUnique n (i + 1) (tail s) (item : acc)
  where
    item = head s

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  print (countUntilUnique 4 0 (head inputLines) [])

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  print (countUntilUnique 14 0 (head inputLines) [])
