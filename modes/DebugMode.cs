using Raylib_cs;
using Vtt.Managers;

namespace Vtt.Modes;

public class DebugMode : DragableMode
{
    int MaxWidthDebugWindow = 0;
    int MaxHeightDebugWindow = 0;

    public override void DrawHUD()
    {
        List<string> strings = new()
        {
            "Camera Position:",
            $"X: {camera.Target.X:F2}, Y: {camera.Target.Y:F2}",
            "Mouse Position (Screen):",
            $"X: {Raylib.GetMouseX()}, Y: {Raylib.GetMouseY()}",
            $"Zoom: {camera.Zoom:F2}x",
        };
        float wheel = Raylib.GetMouseWheelMove();
        if (wheel > 0) strings.Add("Scroll: UP");
        else if (wheel < 0) strings.Add("Scroll: DOWN");
        strings.Add($"FPS: {Raylib.GetFPS()}");
        DrawDebugText(strings);
    }

    void DrawDebugText(List<string> texts)
    {
        MaxWidthDebugWindow = Math.Max(texts.Select(text => Raylib.MeasureText(text, 22)).Max(), MaxWidthDebugWindow);
        MaxHeightDebugWindow = Math.Max((texts.Count + 1) * 20, MaxHeightDebugWindow);
        Raylib.DrawRectangle(10, 10, MaxWidthDebugWindow, MaxHeightDebugWindow, new Color(
            102, 191, 255, 100
        ));

        for (int i = 0; i < texts.Count; i++)
        {
            Raylib.DrawText(texts[i], 20, (i + 1) * 20, 20, Color.White);
        }
    }

    public override void DrawObjects()
    {
        // Draw origin marker
        Raylib.DrawCircle(0, 0, 10, Color.Red);
    }
}