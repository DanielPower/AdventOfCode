module Day3
  ( part1,
    part2,
  )
where

import Data.Bifunctor (first)
import Data.Char (isAsciiLower, isAsciiUpper, ord)
import Data.List.Split (chunksOf)
import Data.Set (Set, fromList, member)

findDuplicate :: Set Char -> [Char] -> Char
findDuplicate itemSet input
  | Data.Set.member item itemSet = item
  | otherwise = findDuplicate itemSet rest
  where
    item = head input
    rest = tail input

charValue :: Char -> Int
charValue char
  | isAsciiLower char = ord char - 96
  | isAsciiUpper char = ord char - 38
  | otherwise = error "Invalid input"

parseBuckets :: String -> (String, String)
parseBuckets line = splitAt splitIndex line
  where
    splitIndex = div (length line) 2

doFindBadge :: String -> [Set Char] -> Char
doFindBadge potentialItems elves
  | all (member item) elves = item
  | otherwise = doFindBadge remainingItems elves
  where
    item = head potentialItems
    remainingItems = drop 1 potentialItems

findBadge :: [String] -> Char
findBadge group = doFindBadge potentialItems remainingElves
  where
    potentialItems = head group
    remainingElves = map fromList (drop 1 group)

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let buckets = map parseBuckets inputLines
  let bucketsWithLeftSet = map (first fromList) buckets
  let duplicates = map (uncurry findDuplicate) bucketsWithLeftSet
  print (sum (map charValue duplicates))

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  let groups = chunksOf 3 inputLines
  let badges = map findBadge groups
  print (sum (map charValue badges))
