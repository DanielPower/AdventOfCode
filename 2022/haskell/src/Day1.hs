module Day1
  ( part1,
    part2,
  )
where

import Data.List (sortBy)
import Data.List.Split (splitWhen)

parseInput :: [String] -> [[String]]
parseInput = splitWhen (== "")

parseElf :: [String] -> Int
parseElf elf = sum (map read elf)

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let elves = parseInput inputLines
  let elfNums = map parseElf elves
  print (maximum elfNums)

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  let elves = parseInput inputLines
  let elfNums = map parseElf elves
  let topElves = take 3 (sortBy (flip compare) elfNums)
  print (sum topElves)
