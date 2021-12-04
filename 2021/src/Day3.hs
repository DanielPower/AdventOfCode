module Day3
  ( part1,
    part2,
  )
where

import Data.Char (digitToInt)
import Data.List (foldl')

numToChar :: Int -> Char
numToChar x
  | x == 0 = '0'
  | x == 1 = '1'
  | otherwise = ' '

flipBit :: Char -> Char
flipBit x
  | x == '0' = '1'
  | x == '1' = '0'
  | otherwise = x

binToDec :: String -> Int
binToDec = foldl' (\acc x -> acc * 2 + digitToInt x) 0

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let inputLength = length inputLines
  let inputNums = map (map digitToInt) inputLines
  let inputSums = foldr1 (zipWith (+)) inputNums
  let gamma = map (numToChar . (`div` inputLength) . (* 2)) inputSums
  let epsilon = map flipBit gamma
  print (binToDec gamma * binToDec epsilon)

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  print "Not implemented"
