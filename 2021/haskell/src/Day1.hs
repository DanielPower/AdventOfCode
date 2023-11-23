module Day1 (part1, part2) where

import Data.List (tails, transpose)

negatives :: [Int] -> [Int]
negatives [] = []
negatives (x : xs)
  | x < 0 = x : negatives xs
  | otherwise = negatives xs

window :: Int -> [Int] -> [[Int]]
window n xs = transpose (take n (tails xs)) -- Source: https://twitter.com/GabriellaG439/status/701460899589017600

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let nums = map read inputLines :: [Int]
  let pairs = zip (init nums) (tail nums)
  let differences = map (uncurry (-)) pairs
  let decreases = length (negatives differences)
  print decreases

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  let nums = map read inputLines :: [Int]
  let groups = window 3 nums
  let sums = map sum groups
  let pairs = zip (init sums) (tail sums)
  let differences = map (uncurry (-)) pairs
  let decreases = length (negatives differences)
  print decreases
