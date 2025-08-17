using Vtt.Datatypes;
using Vtt.Managers;

namespace Vtt.Modes;

public class CampaignListMode : Mode // TODO: Implement this and then move core to ListMode abstract class
{
    Campaign[] campaigns;

    public CampaignListMode()
    {
        // TODO: Temp data. Replace with actual data fetching later
        string imageToken = ImageManager.getInstance().LoadImage("logo.png");

        campaigns = [
            new Campaign("Campaign 1", imageToken),
            new Campaign("Campaign 2", imageToken),
            new Campaign("Campaign 3", imageToken)
        ];
    }

    public override void Draw()
    {

    }

    public override void Update(float deltaTime)
    {

    }
}