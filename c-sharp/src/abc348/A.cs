namespace c_sharp.abc348;

public class A
{
    public static void Run()
    {
        var n = int.Parse(Console.ReadLine()!);
        var ans = string.Join("",Enumerable.Range(0, n).Select(i=>(i+1)%3==0? 'x':'o'));
        Console.WriteLine(ans);
    }
}