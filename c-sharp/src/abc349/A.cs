namespace c_sharp.abc349;

public class A
{
    public static void Run()
    {
        var _ = int.Parse(Console.ReadLine());
        var ans = Console.ReadLine().Split(' ').Select(int.Parse).Sum();
        Console.WriteLine(-ans);
    }
}