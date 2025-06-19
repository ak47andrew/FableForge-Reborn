using Raylib_cs;
using System.Numerics;

namespace Vtt;

public static class State
{
    public static Camera2D camera;
    public static Vector2 dragStartPosition;
    public static Vector2 dragStartCameraTarget;
    public static bool isDragging = false;
#pragma warning disable CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider adding the 'required' modifier or declaring as nullable.
    public static Modes.Mode mode;
#pragma warning restore CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider adding the 'required' modifier or declaring as nullable.
}