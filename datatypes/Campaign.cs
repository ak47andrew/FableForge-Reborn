namespace Vtt.Datatypes;

public class Campaign
{
    public string Name;
    public string Picture;
    private readonly List<Character> characters;
    private readonly List<Scene> scenes;

    public Campaign(string name, string token) // Maybe path and not token?.. I think I'd need to rethink/rewrite ImageManager overall
    {
        Name = name;
        Picture = token;
        characters = new List<Character>();
        scenes = new List<Scene>();
    }

    public Character[] GetCharacters() => characters.ToArray();
    public void AddCharacter(Character character) => characters.Add(character);
    public void RemoveCharacter(Character character) => characters.Remove(character);

    public Scene[] GetScenes() => scenes.ToArray();
    public void AddScene(Scene scene) => scenes.Add(scene);
    public void RemoveScene(Scene scene) => scenes.Remove(scene);
}
