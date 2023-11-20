module Day6 (part1, part2) where

import Data.Foldable (Foldable (toList))
import Data.List.Split (splitWhen)
import Data.Sequence (Seq, adjust, fromList, replicate)

passDay :: [Int] -> [Int]
passDay state = do
  let dead = head state
  let (front, back) = splitAt 6 (tail state)
  front ++ [dead + head back] ++ tail back ++ [dead]

transform :: Seq Int -> [Int] -> Seq Int
transform acc [] = acc
transform acc state = transform (adjust (+ 1) (head state) acc) (tail state)

getIterator :: [String] -> [[Int]]
getIterator inputLines = do
  let initialState = map (read :: String -> Int) (splitWhen (== ',') (head inputLines))
  let transformedState = toList (transform (Data.Sequence.replicate 9 0) initialState)
  iterate passDay transformedState

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  print (sum (getIterator inputLines !! 80))

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  print (sum (getIterator inputLines !! 256))
