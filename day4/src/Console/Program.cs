using Domain.ScratchCard;
using Domain.ScratchCardPairCollection;
using System.IO;
using System.Runtime.InteropServices;

void part1()
{
    string filePath = "Resources/input.txt";
    string content = File.ReadAllText(filePath);

    IScratchCardPairParser parser = new ScratchCardPairParser();
    int total = 0;

    foreach (string line in content.Split('\n'))
    {
        if (line == "")
        {
            continue;
        }
        var pair = parser.Parse(line);
        var score = pair.GetScore();
        total += score;
        Console.WriteLine($"{string.Join(' ', pair.YourNumbers.Numbers)} => {score}");
    }

    Console.WriteLine(total);
}

void part2()
{
    string filePath = "Resources/input.txt";
    string content = File.ReadAllText(filePath);

    List<ScratchCardPair> pairs = new List<ScratchCardPair>();
    IScratchCardPairParser parser = new ScratchCardPairParser();
    foreach (string line in content.Split('\n'))
    {
        if (line == "")
        {
            continue;
        }
        var pair = parser.Parse(line);
        pairs.Add(pair);
    }

    var collection = new ScratchCardPairCollection(pairs);
    var copySum = collection.MakeCopiesAndGetSum();
    Console.WriteLine(copySum.ToString());
}

part1();
part2();
