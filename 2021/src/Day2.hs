module Day2
  ( part1,
    part2,
  )
where

addPair :: (Int, Int) -> (Int, Int) -> (Int, Int)
addPair (a, b) (x, y) = (a + x, b + y)

parseInputWords :: [String] -> (String, Int)
parseInputWords [a, b] = (a, read b :: Int)
parseInputWords _ = ("I assume data good", 0)

displacement :: [(String, Int)] -> (Int, Int)
displacement [] = (0, 0)
displacement ((direction, value) : xs)
  | direction == "down" = addPair (0, value) (displacement xs)
  | direction == "up" = addPair (0, - value) (displacement xs)
  | direction == "forward" = addPair (value, 0) (displacement xs)
  | direction == "backward" = addPair (- value, 0) (displacement xs)
  | otherwise = displacement xs

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let inputWords = map words inputLines
  let inputPairs = map parseInputWords inputWords
  let (distance, depth) = displacement inputPairs
  print (distance * depth)

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  print "Not implemented"
