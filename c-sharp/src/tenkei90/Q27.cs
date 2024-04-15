namespace c_sharp.tenkei90;

public class Q27
{
    public static void Run()
    {
        var n = int.Parse(Console.ReadLine());
        var users = new Dictionary<string, int>();
        var names = new List<string>();
        for (var i = 0; i < n; i++)
        {
            var name = Console.ReadLine();
            if (users.ContainsKey(name)) continue;
            users.Add(name, i + 1);
            names.Add(name);
        }

        names.ForEach(name => Console.WriteLine(users[name]));
    }
}