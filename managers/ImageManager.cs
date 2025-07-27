using Raylib_cs;

namespace Vtt.Managers;

public class ImageManager
{
    private static ImageManager? instance;

    private Dictionary<string, Texture2D> textureCache = new Dictionary<string, Texture2D>();
    private Dictionary<string, string> aliases = new Dictionary<string, string>();

    private ImageManager()
    {
        
    }

    public static ImageManager getInstance()
    {
        instance ??= new ImageManager();
        return instance;
    }

    public string LoadImage(string imagePath)
    {
        string key = imagePath.Replace("/", "-").Replace("\\", "-");
        if (textureCache.ContainsKey(key))
        {
            return key;
        }

        Image image = Raylib.LoadImage(Path.Combine("resources", imagePath));
        Texture2D texture = Raylib.LoadTextureFromImage(image);
        Raylib.UnloadImage(image); // Image no longer needed

        textureCache[key] = texture;
        return key;
    }

    public void CreateAlias(string alias, string initKey)
    {
        aliases[alias] = initKey;
    }

    public Texture2D? GetTexture(string key)
    {
        if (aliases.ContainsKey(key))
        {
            key = aliases[key];
        }
        if (textureCache.ContainsKey(key))
        {
            return textureCache[key];
        }
        Console.WriteLine($"[WARNING] Texture not found: {key}");
        return null;
    }

    public void UnloadAll()
    {
        foreach (var texture in textureCache.Values)
            Raylib.UnloadTexture(texture);
        textureCache.Clear();
    }
}
