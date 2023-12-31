using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Domain.ScratchCard
{
    public sealed record ScratchCardPair(ScratchCard WinningNumbers, ScratchCard YourNumbers)
    {
        private HashSet<int> WinningNumbersYouHave()
        {
            return YourNumbers.AlsoContainedIn(WinningNumbers);
        }

        public int GetCopyCount()
        {
            var winningNumbersYouHave = WinningNumbersYouHave();
            var count = winningNumbersYouHave.Count();
            return count;
        }

        public int GetScore()
        {
            var count = GetCopyCount();
            if (count < 1)
            {
                return 0;
            }
            var result = Math.Pow(2, (double)count - 1);
            return (int)result;
        }
    }
}
