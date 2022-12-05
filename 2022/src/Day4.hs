module Day4
  ( part1,
    part2,
  )
where

import Data.Char (digitToInt)
import Data.List.Split (chunksOf, splitWhen)

type Schedule = (Int, Int)

parseSchedule :: String -> Schedule
parseSchedule schedule = (a, b)
  where
    pair = splitWhen (== '-') schedule
    a = read (head pair)
    b = read (last pair)

parseGroup :: String -> (Schedule, Schedule)
parseGroup group = (parseSchedule (head members), parseSchedule (last members))
  where
    members = splitWhen (== ',') group

areSchedulesFullyRedundant :: (Schedule, Schedule) -> Bool
areSchedulesFullyRedundant (s1, s2)
  | fst s1 == fst s2 = True
  | snd s1 == snd s2 = True
  | otherwise = snd b <= snd a
  where
    (a, b) = if fst s1 < fst s2 then (s1, s2) else (s2, s1)

areSchedulesRedundant :: (Schedule, Schedule) -> Bool
areSchedulesRedundant (s1, s2) = fst b <= snd a
  where
    (a, b) = if fst s1 < fst s2 then (s1, s2) else (s2, s1)

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let groups = map parseGroup inputLines
  let redundancies = filter areSchedulesFullyRedundant groups
  print (length redundancies)

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  let groups = map parseGroup inputLines
  let redundancies = filter areSchedulesRedundant groups
  print (length redundancies)
