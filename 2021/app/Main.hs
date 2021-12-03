module Main where

import Day1
import System.Environment

days =
  [ ("day1.txt", [Day1.part1, Day1.part2])
  ]

main :: IO ()
main = do
  args <- getArgs
  let day = (read (head args) :: Int) - 1
  let (file, solutions) = days !! day
  let part = (read (args !! 1) :: Int) - 1
  let solution = solutions !! part
  solution (readFile file)
