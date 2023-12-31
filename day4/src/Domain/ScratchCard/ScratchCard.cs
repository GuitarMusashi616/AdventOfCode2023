using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Domain.ScratchCard
{
    public sealed record ScratchCard(HashSet<int> Numbers)
    {
        public HashSet<int> AlsoContainedIn(ScratchCard other)
        {
            return Numbers.Where(x => other.Contains(x)).ToHashSet();
        }

        public bool Contains(int i)
        {
            return Numbers.Contains(i);
        }
    }
}
