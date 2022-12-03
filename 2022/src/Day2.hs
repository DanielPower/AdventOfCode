module Day2
  ( part1,
    part2,
  )
where

import Data.List.Split (splitOn)

data Move = Rock | Paper | Scissors deriving (Eq, Enum)

data Result = Win | Draw | Lose

parseMove :: Char -> Move
parseMove 'A' = Rock
parseMove 'X' = Rock
parseMove 'B' = Paper
parseMove 'Y' = Paper
parseMove 'C' = Scissors
parseMove 'Z' = Scissors
parseMove _ = error "Invalid input"

parseInputLine :: String -> (Move, Move)
parseInputLine line = (parseMove opponent, parseMove player)
  where
    moves = splitOn " " line
    opponent = head line
    player = last line

parseInput :: [String] -> [(Move, Move)]
parseInput = map parseInputLine

moveScore :: Move -> Int
moveScore Rock = 1
moveScore Paper = 2
moveScore Scissors = 3

resultScore :: Result -> Int
resultScore Win = 6
resultScore Draw = 3
resultScore Lose = 0

roundResult :: (Move, Move) -> Result
roundResult (opponent, player)
  | opponent == player = Draw
  | opponent == dominantMove player = Lose
  | player == dominantMove opponent = Win
  | otherwise = error "Impossible (hopefully)"

dominantMove :: Move -> Move
dominantMove Scissors = Rock
dominantMove move = succ move

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let moves = parseInput inputLines
  let results = map roundResult moves
  let moveScoreSum = sum (map (moveScore . snd) moves)
  let resultScoreSum = sum (map resultScore results)
  print (moveScoreSum + resultScoreSum)

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  print "Not implemented"
