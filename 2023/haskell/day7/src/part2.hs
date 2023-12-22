module Main (main) where

import Data.List (sortBy, sortOn)
import Data.Map (Map)
import qualified Data.Map as Map

data Card = Jack | Two | Three | Four | Five | Six | Seven | Eight | Nine | Ten | Queen | King | Ace
  deriving (Eq, Ord, Show, Read, Enum)

data Hand = HighCard | Pair | TwoPair | ThreeOfAKind | FullHouse | FourOfAKind | FiveOfAKind
  deriving (Eq, Ord, Show, Read, Enum)

sortDesc :: (Ord a) => [a] -> [a]
sortDesc = sortBy (flip compare)

categorySize :: Int
categorySize = (13 ^ (5 :: Int)) + 13 + 12 * 5

cardMap :: [Card] -> Map Card Int
cardMap = foldr (\c -> Map.insertWith (+) c 1) Map.empty

kicker :: [Card] -> Int
kicker cards = sum $ zipWith weightedValue [4, 3 .. 0] cards
  where
    weightedValue :: Int -> Card -> Int
    weightedValue index card = 13 ^ index * fromEnum card

upgradeHand :: Int -> Hand -> Hand
upgradeHand 0 hand = hand
upgradeHand _ FiveOfAKind = FiveOfAKind
upgradeHand _ FourOfAKind = FiveOfAKind
upgradeHand n FullHouse = upgradeHand (n - 1) FourOfAKind
upgradeHand n ThreeOfAKind = upgradeHand (n - 1) FourOfAKind
upgradeHand n Pair = upgradeHand (n - 1) ThreeOfAKind
upgradeHand n TwoPair = upgradeHand (n - 1) FullHouse
upgradeHand n HighCard = upgradeHand (n - 1) Pair

identifyHand :: [Card] -> Hand
identifyHand cards
  | [5] <- counts = FiveOfAKind
  | (4 : _) <- counts = upgradeHand jCount FourOfAKind
  | (3 : 2 : _) <- counts = upgradeHand jCount FullHouse
  | (3 : _) <- counts = upgradeHand jCount ThreeOfAKind
  | (2 : 2 : _) <- counts = upgradeHand jCount TwoPair
  | (2 : _) <- counts = upgradeHand jCount Pair
  | otherwise = upgradeHand jCount HighCard
  where
    counts = sortDesc (Map.elems (cardMap (filter (/= Jack) cards)))
    jCount = Map.findWithDefault 0 Jack (cardMap cards)

handValue :: [Card] -> Int
handValue cards = categorySize * fromEnum (identifyHand cards) + kicker cards

charToCard :: Char -> Card
charToCard '2' = Two
charToCard '3' = Three
charToCard '4' = Four
charToCard '5' = Five
charToCard '6' = Six
charToCard '7' = Seven
charToCard '8' = Eight
charToCard '9' = Nine
charToCard 'T' = Ten
charToCard 'J' = Jack
charToCard 'Q' = Queen
charToCard 'K' = King
charToCard 'A' = Ace
charToCard _ = error "Invalid card"

parseLine :: String -> ([Card], Int)
parseLine line = (hand, bid)
  where
    [handString, bidString] = words line
    hand = map charToCard handString
    bid = read bidString :: Int

main :: IO ()
main = do
  input <- getContents
  let strings = lines input
  let (hands, bids) = unzip (map parseLine strings)
  let handValues = map handValue hands
  let handValuesWithBids = zip handValues bids
  let sortedHandValuesWithBids = sortOn fst handValuesWithBids
  let (_, sortedBids) = unzip sortedHandValuesWithBids
  let rankedBids = zip sortedBids [(1 :: Int) ..]
  let totalWinnings = sum (map (uncurry (*)) rankedBids)
  print totalWinnings
