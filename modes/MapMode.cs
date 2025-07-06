using Raylib_cs;
using System.Numerics;
using Vtt.Utils;

namespace Vtt.Modes;

public class MapMode : Mode
{
    Camera2D camera;
    bool isDragging;
    Vector2 dragStartPosition;
    Vector2 dragStartCameraTarget;

    public MapMode()
    {
        camera = new Camera2D();
        camera.Target = Vector2.Zero;
        camera.Offset = new Vector2(Settings.SCREEN.X / 2, Settings.SCREEN.Y / 2); // Center camera
        camera.Rotation = 0.0f;
        camera.Zoom = 1.0f;

        isDragging = false;
    }

    public override void Draw()
    {
        using (new Mode2DContext(camera))
        {
            DrawObjects();
        }
        
        DrawHUD();
    }

    public override void Update(float deltaTime)
    {
        UpdateCameraDrag();
    }

    void UpdateCameraDrag()
    {
        if (Raylib.IsMouseButtonPressed(MouseButton.Right))
        {
            // Start dragging
            isDragging = true;
            dragStartPosition = Raylib.GetMousePosition();
            dragStartCameraTarget = camera.Target;
        }
        else if (Raylib.IsMouseButtonReleased(MouseButton.Right))
        {
            // Stop dragging
            isDragging = false;
        }

        float wheel = Raylib.GetMouseWheelMove();

        if (wheel != 0)
        {
            Vector2 mouseWorldPos = Raylib.GetScreenToWorld2D(Raylib.GetMousePosition(), camera);

            camera.Zoom += wheel * Settings.ZOOM_SPEED;
            camera.Zoom = Math.Clamp(camera.Zoom, Settings.MIN_ZOOM, Settings.MAX_ZOOM);

            Vector2 newMouseWorldPos = Raylib.GetScreenToWorld2D(Raylib.GetMousePosition(), camera);

            camera.Target += mouseWorldPos - newMouseWorldPos;
        }

        if (isDragging)
        {
            // Calculate movement delta in screen space
            Vector2 currentMousePos = Raylib.GetMousePosition();
            Vector2 delta = currentMousePos - dragStartPosition;

            // Convert screen delta to world delta (reverse direction)
            delta /= camera.Zoom; // Adjust for zoom
            camera.Target = dragStartCameraTarget - delta;
        }
    }

    public void DrawHUD()
    {
        if (Settings.DEBUG)
        {
            // Draw fixed GUI elements (screen coordinates)
#pragma warning disable CS0162 // Unreachable code detected
            Raylib.DrawRectangle(10, 10, 200, 140, Color.SkyBlue);
            Raylib.DrawText("Camera Position:", 20, 20, 20, Color.DarkBlue);
            Raylib.DrawText($"X: {camera.Target.X:F2}, Y: {camera.Target.Y:F2}", 20, 50, 20, Color.DarkBlue);
            Raylib.DrawText($"Zoom: {camera.Zoom:F2}x", 20, 80, 20, Color.DarkBlue);
            float wheel = Raylib.GetMouseWheelMove();
            if (wheel > 0) Raylib.DrawText("Scroll: UP", 20, 120, 20, Color.DarkGray);
            else if (wheel < 0) Raylib.DrawText("Scroll: DOWN", 20, 120, 20, Color.Red);
#pragma warning restore CS0162 // Unreachable code detected
        }
    }

    public void DrawObjects()
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