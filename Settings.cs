using System.Numerics;

namespace Vtt;

public static class Settings
{
    public const float ZOOM_SPEED = 0.05f;
    public const float MIN_ZOOM = 0.1f;
    public const float MAX_ZOOM = 2f;
    public const bool DEBUG = true;
    public const int BASE_UI_SIZE = 64;
    public static Vector2 SCREEN = new Vector2(1280, 720);
}