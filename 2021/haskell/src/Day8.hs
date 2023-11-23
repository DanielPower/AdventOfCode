module Day8 (part1, part2) where

import Data.List (intersect)
import Data.List.Split (splitWhen)

initialPass :: String -> String -> [String] -> (String, String)
initialPass one four [] = (one, four)
initialPass one four signals
  | length signal == 2 = initialPass signal four remaining
  | length signal == 4 = initialPass one signal remaining
  | otherwise = initialPass one four remaining
  where
    signal = head signals
    remaining = tail signals

decode5 :: String -> String -> String -> Char
decode5 one four signal
  | length (signal `intersect` four) == 2 = '2'
  | length (signal `intersect` one) == 2 = '3'
  | otherwise = '5'

decode6 :: String -> String -> String -> Char
decode6 one four signal
  | length (signal `intersect` four) == 4 = '9'
  | length (signal `intersect` one) == 2 = '0'
  | otherwise = '6'

decode :: String -> String -> String -> Char
decode one four signal
  | length signal == 2 = '1'
  | length signal == 3 = '7'
  | length signal == 4 = '4'
  | length signal == 5 = decode5 one four signal
  | length signal == 6 = decode6 one four signal
  | otherwise = '8'

part1 :: IO String -> IO ()
part1 input = do
  inputLines <- lines <$> input
  let values = concatMap (splitWhen (== ' ') . last . splitWhen (== '|')) inputLines
  print (length (filter (\x -> x < 5 || x == 7) (map length values)))

part2 :: IO String -> IO ()
part2 input = do
  inputLines <- lines <$> input
  let parsedLines = map (map (filter (/= "") . splitWhen (== ' ')) . splitWhen (== '|')) inputLines
  let oneFours = map (\[signals, outputs] -> (initialPass "" "" signals, outputs)) parsedLines
  let outputStrings = map (\((one, four), q) -> map (decode one four) q) oneFours
  print (sum (map (read :: String -> Int) outputStrings))
