using Raylib_cs;

namespace Vtt.Modes;

public class Map : Mode
{
    public override void DrawHUD()
    {
        if (Settings.DEBUG)
        {
            // Draw fixed GUI elements (screen coordinates)
#pragma warning disable CS0162 // Unreachable code detected
            Raylib.DrawRectangle(10, 10, 200, 140, Color.SkyBlue);
            Raylib.DrawText("Camera Position:", 20, 20, 20, Color.DarkBlue);
            Raylib.DrawText($"X: {State.camera.Target.X:F2}, Y: {State.camera.Target.Y:F2}", 20, 50, 20, Color.DarkBlue);
            Raylib.DrawText($"Zoom: {State.camera.Zoom:F2}x", 20, 80, 20, Color.DarkBlue);
            float wheel = Raylib.GetMouseWheelMove();
            if (wheel > 0) Raylib.DrawText("Scroll: UP", 20, 120, 20, Color.DarkGray);
            else if (wheel < 0) Raylib.DrawText("Scroll: DOWN", 20, 120, 20, Color.Red);
#pragma warning restore CS0162 // Unreachable code detected
        }
    }

    public override void DrawObjects()
    {
        // Draw grid (world coordinates)
        for (int x = -1000; x <= 1000; x += 100)
        {
            Raylib.DrawLine(x, -1000, x, 1000, Color.DarkGray);
        }
        for (int y = -1000; y <= 1000; y += 100)
        {
            Raylib.DrawLine(-1000, y, 1000, y, Color.DarkGray);
        }

        // Draw origin marker
        Raylib.DrawCircle(0, 0, 10, Color.Red);
    }
}