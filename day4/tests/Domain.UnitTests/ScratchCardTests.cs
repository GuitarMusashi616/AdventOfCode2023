using Domain.ScratchCard;
using FluentAssertions;

namespace Domain.UnitTests;

public class ScratchCardTests
{
    [Fact]
    public void Create_Should_ReturnValidScratchCardPair_WhenParsingLine()
    {
        string line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        IScratchCardPairParser parser = new ScratchCardPairParser();

        var cardPair = parser.Parse(line);

        cardPair.WinningNumbers.Numbers.Should().Contain(new[] {41, 48, 83, 86, 17});
        cardPair.YourNumbers.Numbers.Should().Contain(new[] {83, 86, 6, 31, 17, 9, 48, 53});
    }

    [Fact]
    public void MakeCopies_Should_ReturnCorrectAnswer_WhenCalled()
    {
    }
}