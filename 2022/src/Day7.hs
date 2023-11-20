module Day7
  ( part1,
    part2,
  )
where

import Data.HashMap (Map)

data Instruction = Cd String | Dir String | Ls deriving (Show)

-- TODO, wrote Day 7 in Javascipt. Will come back to it eventually.
parseInstruction :: String -> Instruction
parseInstruction s
  | command == "cd" = Cd (last tokens)
  | command == "dir" = Dir (last tokens)
  | command == "ls" = Ls
  | otherwise = error "Bad input"
  where
    tokens = words s
    command = head tokens

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  print "Not implemented"

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  print "Not implemented"
