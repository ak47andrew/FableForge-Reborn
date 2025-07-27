using Raylib_cs;
using System.Numerics;
using Vtt.Utils;
namespace Vtt.Modes;

public abstract class DragableMode : Mode
{
    protected Camera2D camera;
    protected bool isDragging;
    protected Vector2 dragStartPosition;
    protected Vector2 dragStartCameraTarget;

    public DragableMode()
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

    public abstract void DrawHUD();
    public abstract void DrawObjects();
}