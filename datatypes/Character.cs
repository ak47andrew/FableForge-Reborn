namespace Vtt.Datatypes;

public class Character
{
    // Okay, let me vent here a little bit
    // D&D is a fucking mess! (Well, not really, I'm just a lil lazy and dumb for that)
    // Like, the whole character fields and stuff will be a nightmare to set up
    // I think that's one of the reasons why this will not be ready until September or later
    // This will take a shitload of work, creating characters by hand and figuring out the fields

    // For now, let's just have a basic character with a name and then I'll finish it up in the end

    public string Name;

    public Character(string name)
    {
        Name = name;
    }
}