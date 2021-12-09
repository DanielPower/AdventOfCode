module Day7 (part1, part2) where

import Data.Foldable (Foldable (toList))
import Data.List.Split (splitWhen)
import Data.Sequence (fromList, index, sort)

triangle :: Int -> Int
triangle x = div (x * (x + 1)) 2

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let initialState = fromList (map (read :: String -> Int) (splitWhen (== ',') (head inputLines)))
  let median = index (sort initialState) (div (length initialState) 2)
  let differences = map (\x -> abs (x - median)) (toList initialState)
  print (sum differences)

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  let initialState = map (read :: String -> Int) (splitWhen (== ',') (head inputLines))
  let mean = div (sum initialState) (length initialState)
  let differences = map (\x -> triangle (abs (x - mean))) initialState
  print (sum differences)
