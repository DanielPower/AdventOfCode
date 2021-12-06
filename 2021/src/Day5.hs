module Day5
  ( part1,
    part2,
  )
where

import Data.List.Split (splitWhen)
import Data.Map (Map, elems, empty, findWithDefault, insert)

isValidVentLine :: [[Int]] -> Bool
isValidVentLine line = do
  let [x1, y1] = head line
  let [x2, y2] = last line
  x1 == x2 || y1 == y2

isValidVentLine2 :: [[Int]] -> Bool
isValidVentLine2 line = do
  let [x1, y1] = head line
  let [x2, y2] = last line
  x1 == x2 || y1 == y2 || (abs (x1 - x2) == abs (y1 - y2))

ventLineToPoints :: [[Int]] -> [(Int, Int)]
ventLineToPoints line
  | x1 == x2 || y1 == y2 = straightLine line
  | otherwise = diagonalLine line
  where
    [x1, y1] = head line
    [x2, y2] = last line

straightLine :: [[Int]] -> [(Int, Int)]
straightLine line = do
  let [x1, y1] = head line
  let [x2, y2] = last line
  let minX = min x1 x2
  let minY = min y1 y2
  let maxX = max x1 x2
  let maxY = max y1 y2
  [(x, y) | x <- [minX .. maxX], y <- [minY .. maxY]]

diagonalLine :: [[Int]] -> [(Int, Int)]
diagonalLine line = do
  let [x1, y1] = head line
  let [x2, y2] = last line
  let minX = min x1 x2
  let minY = min y1 y2
  let maxX = max x1 x2
  let maxY = max y1 y2
  zip [x1, x1 + (signum $ x2 - x1)..x2] [y1, y1 + (signum $ y2 - y1)..y2] -- Source: https://stackoverflow.com/a/30063912

countVentLines :: Map (Int, Int) Int -> [(Int, Int)] -> Map (Int, Int) Int
countVentLines counts [] = counts
countVentLines counts points = do
  let point = head points
  let newCountForPoint = findWithDefault (0 :: Int) point counts + 1
  let newCounts = insert point newCountForPoint counts
  countVentLines newCounts (tail points)

parseInput :: [String] -> ([[Int]] -> Bool) -> [(Int, Int)]
parseInput inputLines lineValidator =
  concatMap
  ventLineToPoints
  ( filter
      lineValidator
      ( map
          ( map
              ( map (read :: String -> Int)
                  . splitWhen (== ',')
              )
              . filter (/= "->")
              . splitWhen (== ' ')
          )
          inputLines
      )
  )

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let ventPoints = parseInput inputLines isValidVentLine
  let ventLineCounts = countVentLines empty ventPoints
  let ventLineIntersections = filter (> 1) (elems ventLineCounts)
  print (length ventLineIntersections)

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  let ventPoints = parseInput inputLines isValidVentLine2
  let ventLineCounts = countVentLines empty ventPoints
  let ventLineIntersections = filter (> 1) (elems ventLineCounts)
  print (length ventLineIntersections)
