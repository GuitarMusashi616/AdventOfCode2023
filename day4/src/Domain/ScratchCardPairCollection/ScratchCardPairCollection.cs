using Domain.ScratchCard;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Domain.ScratchCardPairCollection
{
    public class ScratchCardPairCollection
    {
        private List<int> cardPairCount;
        public List<ScratchCardPair> CardPairs { get; init; }

        public ScratchCardPairCollection(List<ScratchCardPair> cardPairs)
        {
            this.CardPairs = cardPairs;
            this.cardPairCount = CardPairs.Select(x => 1).ToList();
        }

        public int MakeCopiesAndGetSum()
        {
            for (int i = 0; i < CardPairs.Count-1; i++)
            {
                int copyCount = CardPairs[i].GetCopyCount();
                UpdateCardPairCount(copyCount, i);
            }
            PrintCardPairCount();
            return cardPairCount.Sum();
        }

        private void UpdateCardPairCount(int copyXSuccessiveCards, int index)
        {
            for (int i = index; i < index + copyXSuccessiveCards; i++)
            {
                cardPairCount[i + 1] += cardPairCount[index];
            }
        }

        private void PrintCardPairCount()
        {
            for (int i = 0; i < cardPairCount.Count; i++)
            {
                Console.WriteLine($"{string.Join(' ', CardPairs[i].YourNumbers.Numbers)} => {cardPairCount[i]} copies");
            }
        }
    }
}
