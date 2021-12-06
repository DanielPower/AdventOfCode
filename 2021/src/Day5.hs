module Day5
  ( part1,
    part2,
  )
where

import Data.List.Split (splitWhen)
import Data.Map (Map, elems, empty, findWithDefault, insert)
import Debug.Trace (trace, traceShow)

isValidVentLine :: [[Int]] -> Bool
isValidVentLine line = do
  let [x1, y1] = head line
  let [x2, y2] = last line
  x1 == x2 || y1 == y2

ventLineToPoints :: [[Int]] -> [(Int, Int)]
ventLineToPoints line = do
  let [x1, y1] = head line
  let [x2, y2] = last line
  let minX = min x1 x2
  let minY = min y1 y2
  let maxX = max x1 x2
  let maxY = max y1 y2
  [(x, y) | x <- [minX .. maxX], y <- [minY .. maxY]]


countVentLines :: Map (Int, Int) Int -> [(Int, Int)] -> Map (Int, Int) Int
countVentLines counts [] = counts
countVentLines counts points = do
  let point = head points
  let newCountForPoint = findWithDefault (0 :: Int) point counts + 1
  let newCounts = insert point newCountForPoint counts
  countVentLines newCounts (tail points)

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let ventPoints =
        concatMap
          ventLineToPoints
          ( filter
              isValidVentLine
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
  let ventLineCounts = countVentLines empty ventPoints
  let ventLineIntersections = filter (>1) (elems ventLineCounts)
  print (length ventLineIntersections)

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  print "Not implemented"
