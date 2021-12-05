module Day4
  ( part1,
    part2,
  )
where

import Data.List.Split (splitWhen)
import Data.List (find)
import Data.Maybe (isJust, fromJust)

bingoLines :: [[(Int, Int)]]
bingoLines =
  map (\v -> [(x, y) | x <- [0 .. 4], y <- [v]]) [0 .. 4]
    ++ map (\v -> [(x, y) | x <- [v], y <- [0 .. 4]]) [0 .. 4]

-- Source: https://stackoverflow.com/a/4981265
wordsWhen :: (Char -> Bool) -> String -> [String]
wordsWhen p s = case dropWhile p s of
  "" -> []
  s' -> w : wordsWhen p s''
    where
      (w, s'') = break p s'

getValue :: (Int, Int) -> [Int] -> Int
getValue (x, y) board = board !! (y * 5 + x)

replaceWhen :: a -> (a -> Bool) -> a -> a
replaceWhen value condition newValue
  | condition value = newValue
  | otherwise = value

dabBoard :: [Int] -> Int -> [Int]
dabBoard board calledValue = map (\value -> replaceWhen value (== calledValue) (-1)) board

dabBoards :: [[Int]] -> Int -> [[Int]]
dabBoards boards number = map (`dabBoard` number) boards

checkLineWin :: [Int] -> [(Int, Int)] -> Bool
checkLineWin board = all (\point -> getValue point board == -1)

checkBoardWin :: [Int] -> Bool
checkBoardWin board = any (checkLineWin board) bingoLines

letTheDabBegin :: [[Int]] -> [Int] -> Int -> ([Int], Int)
letTheDabBegin boards numbers lastNumber
  | isJust (find checkBoardWin boards) = (fromJust (find checkBoardWin boards), lastNumber)
  | otherwise = letTheDabBegin (dabBoards boards (head numbers)) (tail numbers) (head numbers)

calculateScore :: [Int] -> Int -> Int
calculateScore board number = sum (filter (>0) board) * number

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let numbers = map (read::String->Int) (wordsWhen (== ',') (head inputLines))
  let boardLines = tail (tail inputLines)
  let boards = map (concatMap (wordsWhen (== ' '))) (splitWhen (== "") boardLines)
  let boards' = map (map (read::String->Int)) boards
  let (winningBoard, winningNumber) = letTheDabBegin boards' numbers 0
  print (calculateScore winningBoard winningNumber)

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  print "Not implemented"
