using Raylib_cs;
using System.Numerics;

public class CameraDragExample
{
    static Camera2D camera;
    static Vector2 dragStartPosition;
    static Vector2 dragStartCameraTarget;
    static bool isDragging = false;
    const float ZOOM_SPEED = 0.05f;
    const float MIN_ZOOM = 0.1f;
    const float MAX_ZOOM = 2f;


    public static void Main()
    {
        const int screenWidth = 800;
        const int screenHeight = 600;

        Raylib.InitWindow(screenWidth, screenHeight, "Camera Drag Example");
        Raylib.SetTargetFPS(60);

        // Initialize camera
        camera = new Camera2D();
        camera.Target = Vector2.Zero;
        camera.Offset = new Vector2(screenWidth / 2, screenHeight / 2); // Center camera
        camera.Rotation = 0.0f;
        camera.Zoom = 1.0f;

        while (!Raylib.WindowShouldClose())
        {
            // Update
            UpdateCameraDrag();

            // Draw
            Raylib.BeginDrawing();
            Raylib.ClearBackground(Color.LightGray);

            // Draw game world with camera transformation
            Raylib.BeginMode2D(camera);
            DrawGameWorld();
            Raylib.EndMode2D();

            // Draw GUI (fixed screen position)
            DrawGUI();

            Raylib.EndDrawing();
        }

        Raylib.CloseWindow();
    }

    static void UpdateCameraDrag()
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

            camera.Zoom += wheel * ZOOM_SPEED;
            camera.Zoom = Math.Clamp(camera.Zoom, MIN_ZOOM, MAX_ZOOM);

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

    static void DrawGameWorld()
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

    static void DrawGUI()
    {
        // Draw fixed GUI elements (screen coordinates)
        Raylib.DrawRectangle(10, 10, 200, 140, Color.SkyBlue);
        Raylib.DrawText("Camera Position:", 20, 20, 20, Color.DarkBlue);
        Raylib.DrawText($"X: {camera.Target.X:F2}, Y: {camera.Target.Y:F2}", 20, 50, 20, Color.DarkBlue);
        Raylib.DrawText($"Zoom: {camera.Zoom:F2}x", 20, 80, 20, Color.DarkBlue);
        float wheel = Raylib.GetMouseWheelMove();
        if (wheel > 0) Raylib.DrawText("Scroll: UP", 20, 120, 20, Color.DarkGray);
        else if (wheel < 0) Raylib.DrawText("Scroll: DOWN", 20, 120, 20, Color.Red);
    }
}