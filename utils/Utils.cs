using Raylib_cs;

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
