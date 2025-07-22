using Raylib_cs;
using Vtt.Managers;

namespace Vtt.Modes;

public class MapMode : DragableMode
{
    public MapMode()
    {
        ImageManager.getInstance().LoadImage("bg.png");
    }

    public override void Update(float deltaTime)
    {
        base.Update(deltaTime);
    }

    public override void DrawHUD()
    {

    }

    public override void DrawObjects()
    {
        Texture2D? tmp = ImageManager.getInstance().GetTexture("bg.png");
        if (!tmp.HasValue)
        {
            return;
        }
        Texture2D tex = tmp.Value;

        Raylib.DrawTexture(tex, 0, 0, Color.White);

        for (int x = 0; x <= tex.Width; x += 100)
        {
            Raylib.DrawLine(x, 0, x, tex.Height, Color.DarkGray);
        }
        for (int y = 0; y <= tex.Height; y += 100)
        {
            Raylib.DrawLine(0, y, tex.Width, y, Color.DarkGray);
        }
    }
}