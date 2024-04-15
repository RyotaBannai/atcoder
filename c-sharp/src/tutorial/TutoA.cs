namespace c_sharp.tutorial;

public class TutoA
{
    public static void run()
    {
        // 整数の入力
        var a = int.Parse(Console.ReadLine());
        // スペース区切りの整数の入力
        var input = Console.ReadLine().Split(' ');
        var b = int.Parse(input[0]);
        var c = int.Parse(input[1]);
        // 文字列の入力
        var s = Console.ReadLine();
        //出力
        var ret = a + b + c + " " + s;
        Console.WriteLine("{0}", ret);
    }
}