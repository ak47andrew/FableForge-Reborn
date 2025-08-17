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
        manager.setMode(new DebugMode());
        
        float returnToDebug = 0;

        while (!Raylib.WindowShouldClose())
        {
            manager.getCurrentMode().Update(Raylib.GetFrameTime());

            if (Settings.DEBUG)
            {                
                if (Raylib.IsKeyDown(KeyboardKey.Enter) && Raylib.IsKeyDown(KeyboardKey.Backspace) && manager.getCurrentMode() is not DebugMode)
                {
                    returnToDebug += Raylib.GetFrameTime();
                    Console.WriteLine($"Returning to DebugMode in {Settings.ENTER_DEBUG_TIME - returnToDebug:F2} seconds...");
                    if (returnToDebug > Settings.ENTER_DEBUG_TIME)
                    {
                        manager.setMode(new DebugMode());
                        returnToDebug = 0;
                    }
                }
                else
                {
                    returnToDebug = 0;
                }
            }

            using (new DrawingContext())
            {
                Raylib.ClearBackground(Color.DarkGray);

                manager.getCurrentMode().Draw();
            }
        }

        ImageManager.getInstance().UnloadAll();

        Raylib.CloseWindow();
    }
}