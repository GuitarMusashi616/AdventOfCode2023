using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Domain.ScratchCard
{
    public sealed class ScratchCardPairParser : IScratchCardPairParser
    {
        public ScratchCardPair Parse(string line)
        {
            var afterColon = GetStrAfterColon(line);
            var splitBar = GetSplitBar(afterColon);
            var cards = GetScratchCards(splitBar);
            if (cards == null || cards.Count != 2)
            {
                throw new ArgumentException($"Wrong number of cards in {splitBar}");
            }

            return new ScratchCardPair(cards[0], cards[1]);
        }

        private List<ScratchCard> GetScratchCards(string[] splitBar)
        {
            var cards = new List<ScratchCard>();
            foreach (var numSplit in splitBar)
            {
                var numbers = SplitNumbers(numSplit.Trim());
                var card = new ScratchCard(numbers);
                cards.Add(card);
            }
            return cards;
        }

        private string[] GetSplitBar(string afterColon)
        {
            var splitBar = afterColon.Split("|");
            if (splitBar == null || splitBar.Length != 2)
            {
                throw new ArgumentException($"Line could not be parsed{afterColon}");
            }
            return splitBar;
        }

        private string GetStrAfterColon(string colonStr)
        {
            var splitColon = colonStr.Split(':');
            if (splitColon == null || splitColon.Length != 2)
            {
                throw new ArgumentException($"Line could not be parsed{colonStr}");
            }
            var afterColon = splitColon[1].Trim();
            return afterColon;
        }

        private HashSet<int> SplitNumbers(string numberLine)
        {
            var result = new HashSet<int>();
            foreach (string numberStr in numberLine.Split())
            {
                var trimmed = numberStr.Trim();
                if (trimmed == "")
                {
                    continue;
                }
                var validNum = int.Parse(trimmed);
                result.Add(validNum);
            }
            return result;
        }
    }
}
