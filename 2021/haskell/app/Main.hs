module Main where

import Day1
import Day2
import Day3
import Day4
import Day5
import Day6
import Day7
import Day8
import System.Environment

days =
  [ ("day1.txt", [Day1.part1, Day1.part2]),
    ("day2.txt", [Day2.part1, Day2.part2]),
    ("day3.txt", [Day3.part1, Day3.part2]),
    ("day4.txt", [Day4.part1, Day4.part2]),
    ("day5.txt", [Day5.part1, Day5.part2]),
    ("day6.txt", [Day6.part1, Day6.part2]),
    ("day7.txt", [Day7.part1, Day7.part2]),
    ("day8.txt", [Day8.part1, Day8.part2])
  ]

main :: IO ()
main = do
  args <- getArgs
  let day = (read (head args) :: Int) - 1
  let (file, solutions) = days !! day
  let part = (read (args !! 1) :: Int) - 1
  let solution = solutions !! part
  solution (readFile ("inputs/" ++ file))
