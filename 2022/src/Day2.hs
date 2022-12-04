module Day2
  ( part1,
    part2,
  )
where

import Data.List.Split (splitOn)

data Move = Rock | Paper | Scissors deriving (Eq)

data Result = Win | Draw | Lose

parseMove :: Char -> Move
parseMove 'A' = Rock
parseMove 'X' = Rock
parseMove 'B' = Paper
parseMove 'Y' = Paper
parseMove 'C' = Scissors
parseMove 'Z' = Scissors
parseMove _ = error "Invalid input"

parseResult :: Char -> Result
parseResult 'X' = Lose
parseResult 'Y' = Draw
parseResult 'Z' = Win
parseResult _ = error "Invalid input"

parseInputLine :: String -> (Move, Move)
parseInputLine line = (parseMove (head line), parseMove (last line))

parseInputLine2 :: String -> (Move, Result)
parseInputLine2 line = (parseMove (head line), parseResult (last line))

moveScore :: Move -> Int
moveScore Rock = 1
moveScore Paper = 2
moveScore Scissors = 3

resultScore :: Result -> Int
resultScore Win = 6
resultScore Draw = 3
resultScore Lose = 0

movesForResult :: (Move, Result) -> (Move, Move)
movesForResult (opponent, Win) = (opponent, dominantMove opponent)
movesForResult (opponent, Draw) = (opponent, opponent)
movesForResult (opponent, Lose) = (opponent, dominantMove (dominantMove opponent))

dominantMove :: Move -> Move
dominantMove Rock = Paper
dominantMove Paper = Scissors
dominantMove Scissors = Rock

roundResult :: (Move, Move) -> Result
roundResult (opponent, player)
  | opponent == player = Draw
  | opponent == dominantMove player = Lose
  | player == dominantMove opponent = Win
  | otherwise = error "Impossible (hopefully)"

computeScore :: [(Move, Move)] -> [Result] -> Int
computeScore moves results = sum (map (moveScore . snd) moves) + sum (map resultScore results)

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let moves = map parseInputLine inputLines
  let results = map roundResult moves
  print (computeScore moves results)

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  let moves = map (movesForResult . parseInputLine2) inputLines
  let results = map roundResult moves
  print (computeScore moves results)
