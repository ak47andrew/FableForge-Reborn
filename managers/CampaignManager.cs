using Vtt.Datatypes;

namespace Vtt.Managers;

public class CampaignManager
{
    private static CampaignManager? instance;

    private CampaignManager()
    {
    }

    public static CampaignManager getInstance()
    {
        if (instance == null)
        {
            instance = new CampaignManager();
        }
        return instance;
    }

    public void Save(Campaign campaign)
    {
        string path = GetPath(campaign.Name);

        using (FileStream fs = File.OpenWrite(path))
        {
            // TODO: Serialize the campaign object to the file stream
        }
    }

    public Campaign? Load(string name)
    {
        string path = GetPath(name);

        if (!File.Exists(path))
        {
            return null;
        }

        using (FileStream fs = File.OpenRead(path))
        {
            // TODO: Deserialize the campaign object from the file stream
            return null; // Replace with the deserialized campaign object
        }
    }

    private string GetPath(string name)
    {
        return Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), "FableForge-Reborn", "campaigns", name + ".ffc");
    }
}
