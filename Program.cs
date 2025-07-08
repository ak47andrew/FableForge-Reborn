using Raylib_cs;
using Vtt.Modes;
using Vtt.Utils;
using Vtt.Managers;

namespace Vtt;

public class VTT
{
    public static void Main()
    {
        Raylib.InitWindow((int)Settings.SCREEN.X, (int)Settings.SCREEN.Y, "FableForge Reborn");
        Raylib.SetTargetFPS(60);

        ModeManager manager = ModeManager.getInstance();
        manager.setMode(new MapMode());

        while (!Raylib.WindowShouldClose())
        {
            manager.getCurrentMode().Update(Raylib.GetFrameTime());

            using (new DrawingContext())
            {
                Raylib.ClearBackground(Color.LightGray);

                manager.getCurrentMode().Draw();
            }
        }

        Raylib.CloseWindow();
    }
}