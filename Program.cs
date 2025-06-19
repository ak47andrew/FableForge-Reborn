using Raylib_cs;
using System.Numerics;

namespace Vtt;

public class VTT
{
    public static void Main()
    {

        Raylib.InitWindow((int)Settings.SCREEN.X, (int)Settings.SCREEN.Y, "FableForge Reborn");
        Raylib.SetTargetFPS(60);

        State.mode = new Modes.Map();

        // Initialize camera
        State.camera = new Camera2D();
        State.camera.Target = Vector2.Zero;
        State.camera.Offset = new Vector2(Settings.SCREEN.X / 2, Settings.SCREEN.Y / 2); // Center camera
        State.camera.Rotation = 0.0f;
        State.camera.Zoom = 1.0f;

        while (!Raylib.WindowShouldClose())
        {
           UpdateCameraDrag();
    
            using (new DrawingContext())
            {
                Raylib.ClearBackground(Color.LightGray);
                
                using (new Mode2DContext(State.camera))
                {
                    DrawGameWorld();
                }
                
                DrawGUI();
            }
        }

        Raylib.CloseWindow();
    }

    static void UpdateCameraDrag()
    {
        if (Raylib.IsMouseButtonPressed(MouseButton.Right))
        {
            // Start dragging
            State.isDragging = true;
            State.dragStartPosition = Raylib.GetMousePosition();
            State.dragStartCameraTarget = State.camera.Target;
        }
        else if (Raylib.IsMouseButtonReleased(MouseButton.Right))
        {
            // Stop dragging
            State.isDragging = false;
        }

        float wheel = Raylib.GetMouseWheelMove();

        if (wheel != 0)
        {
            Vector2 mouseWorldPos = Raylib.GetScreenToWorld2D(Raylib.GetMousePosition(), State.camera);

            State.camera.Zoom += wheel * Settings.ZOOM_SPEED;
            State.camera.Zoom = Math.Clamp(State.camera.Zoom, Settings.MIN_ZOOM, Settings.MAX_ZOOM);

            Vector2 newMouseWorldPos = Raylib.GetScreenToWorld2D(Raylib.GetMousePosition(), State.camera);

            State.camera.Target += mouseWorldPos - newMouseWorldPos;
        }


        if (State.isDragging)
        {
            // Calculate movement delta in screen space
            Vector2 currentMousePos = Raylib.GetMousePosition();
            Vector2 delta = currentMousePos - State.dragStartPosition;

            // Convert screen delta to world delta (reverse direction)
            delta /= State.camera.Zoom; // Adjust for zoom
            State.camera.Target = State.dragStartCameraTarget - delta;
        }
    }

    static void DrawGameWorld()
    {
        State.mode.DrawObjects();
    }

    static void DrawGUI()
    {
        State.mode.DrawHUD();
    }
}