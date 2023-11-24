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
  [ ("01.in", [Day1.part1, Day1.part2]),
    ("02.in", [Day2.part1, Day2.part2]),
    ("03.in", [Day3.part1, Day3.part2]),
    ("04.in", [Day4.part1, Day4.part2]),
    ("05.in", [Day5.part1, Day5.part2]),
    ("06.in", [Day6.part1, Day6.part2]),
    ("07.in", [Day7.part1, Day7.part2]),
    ("08.in", [Day8.part1, Day8.part2])
  ]

main :: IO ()
main = do
  args <- getArgs
  let day = (read (head args) :: Int) - 1
  let (file, solutions) = days !! day
  let part = (read (args !! 1) :: Int) - 1
  let solution = solutions !! part
  solution (readFile ("../inputs/" ++ file))
