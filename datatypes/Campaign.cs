namespace Vtt.Datatypes;

public class Campaign
{
    public string Name;
    private readonly List<Character> characters;
    private readonly List<Scene> scenes;

    public Campaign(string name)
    {
        Name = name;
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
