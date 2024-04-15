namespace c_sharp.abc349;

public class B
{
    public static void Run()
    {
        var st = Console.ReadLine()!;
        var counts = new Dictionary<char, int>();
        foreach (var c in st)
            if (counts.ContainsKey(c))
                counts[c]++;
            else counts.Add(c, 1);

        var d = new Dictionary<int, int>();
        foreach (var item in counts.Values)
            if (d.ContainsKey(item))
                d[item]++;
            else d.Add(item, 1);

        var ans = d.Values.All(count => count == 0 | count == 2);
        Console.WriteLine(ans ? "Yes" : "No");
    }
}