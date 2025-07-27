using Raylib_cs;
using System.Numerics;

namespace Vtt.Utils;

public class DrawingContext : IDisposable
{
    public DrawingContext() => Raylib.BeginDrawing();
    public void Dispose() => Raylib.EndDrawing();
}

public class Mode2DContext : IDisposable
{
    private readonly Camera2D _camera;

    public Mode2DContext(Camera2D camera)
    {
        _camera = camera;
        Raylib.BeginMode2D(camera);
    }

    public void Dispose() => Raylib.EndMode2D();
}

public static class Utils
{
    public static Color ColorLerp(Color a, Color b, float t)
    {
        return new Color(
            (byte)(a.R + (b.R - a.R) * t),
            (byte)(a.G + (b.G - a.G) * t),
            (byte)(a.B + (b.B - a.B) * t),
            (byte)(a.A + (b.A - a.A) * t)
        );
    }

    public static Vector2 GetWorldMousePosition(Camera2D camera)
    {
        Vector2 screenMousePos = Raylib.GetMousePosition();
        return Raylib.GetScreenToWorld2D(screenMousePos, camera);
    }
}